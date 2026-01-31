//! # Validator Extensions
//!
//! 扩展 validator crate，支持 `#[validate(must_be_true(...))]` 语法
//!
//! ## 使用示例
//!
//! ```rust
//! use validator_ext::must_be_true_validator;
//!
//! #[must_be_true_validator]
//! #[derive(Debug, Clone, Validate)]
//! pub struct RegisterRequest {
//!     #[validate(length(min = 3, max = 50))]
//!     pub username: String,
//!
//!     #[validate(must_be_true(message = "必须同意用户协议"))]
//!     pub accept_terms: bool,
//! }
//!
//! // 使用生成的验证方法
//! let request = RegisterRequest { username: "test".to_string(), accept_terms: true };
//! request.validate_accept_terms().unwrap();
//! ```

use proc_macro::TokenStream;
use std::str::FromStr;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Field, Fields, parse_macro_input, parse::Parser};

/// # 自定义属性宏
///
/// 为结构体生成验证方法，并从 `#[validate(...)]` 属性中移除 `must_be_true`，
/// 避免与 `validator::Validate` 冲突。
///
/// ## 功能
/// - 扫描结构体中所有带有 `#[validate(..., must_be_true(...), ...)]` 的字段
/// - 从 `#[validate(...)]` 中移除 `must_be_true` 部分
/// - 为每个字段生成对应的验证方法
/// - 其他验证规则（如 length, email 等）保持不变，由 `validator::Validate` 处理
///
/// ## 参数
/// - `_attr`: 属性宏的参数（当前未使用）
/// - `item`: 被标注的结构体
///
/// ## 返回
/// 扩展后的 TokenStream，包含修改后的结构体定义和生成的验证方法实现
///
/// ## 示例
///
/// ```rust
/// #[must_be_true_validator]
/// #[derive(Debug, Clone, Validate)]
/// pub struct Request {
///     #[validate(must_be_true(message = "必须同意用户协议"))]
///     pub accept_terms: bool,
/// }
///
/// // 自动生成的方法：
/// // fn validate_accept_terms(&self) -> Result<(), validator::ValidationError>
/// ```
#[proc_macro_attribute]
pub fn must_be_true_validator(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);

    // 处理字段：提取 must_be_true 并从 validate 属性中移除
    let must_be_true_fields = process_fields(&mut input);

    let struct_name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // 生成最终代码
    let expanded = quote! {
        #input

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#must_be_true_fields)*
        }
    };

    TokenStream::from(expanded)
}

/// # Derive 宏版本
///
/// 提供一个 derive 宏版本的实现，功能与属性宏版本相同。
#[proc_macro_derive(ValidateWithMustBeTrue)]
pub fn validate_with_must_be_true(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);

    let must_be_true_fields = process_fields(&mut input);

    let struct_name = input.ident.clone();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let expanded = quote! {
        #input

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#must_be_true_fields)*
        }
    };

    TokenStream::from(expanded)
}

/// # 处理字段
///
/// 遍历结构体的所有字段，提取 `must_be_true` 并从 `validate` 属性中移除。
fn process_fields(input: &mut DeriveInput) -> Vec<proc_macro2::TokenStream> {
    let mut validation_methods = Vec::new();
    let mut field_counter = 0;

    if let Data::Struct(ref mut data_struct) = input.data {
        if let Fields::Named(ref mut named_fields) = data_struct.fields {
            for field in named_fields.named.iter_mut() {
                // 尝试从 validate 属性中提取并移除 must_be_true
                if let Some(message) = extract_and_remove_must_be_true(field) {
                    let field_name = field.ident.as_ref().unwrap();
                    let fn_name = generate_validation_fn_name(field_name, &mut field_counter);

                    validation_methods.push(quote! {
                        fn #fn_name(&self) -> Result<(), validator::ValidationError> {
                            if !self.#field_name {
                                return Err(validator::ValidationError::new("must_be_true")
                                    .with_message(std::borrow::Cow::from(#message)));
                            }
                            Ok(())
                        }
                    });
                }
            }
        }
    }

    validation_methods
}

/// # 提取并移除 must_be_true
///
/// 从字段的 `#[validate(...)]` 属性中提取 `must_be_true` 的消息，
/// 并从属性中移除它。
///
/// ## 参数
/// - `field`: 要处理的可变字段引用
///
/// ## 返回
/// - `Some(message)`: 如果找到 must_be_true，返回消息
/// - `None`: 如果没找到 must_be_true
fn extract_and_remove_must_be_true(field: &mut Field) -> Option<String> {
    for i in (0..field.attrs.len()).rev() {
        let attr = &field.attrs[i];

        // 只处理 validate 属性
        if attr.path().is_ident("validate") {
            // 将属性转换为字符串进行处理
            let attr_str = attr.to_token_stream().to_string();

            // 检查是否包含 must_be_true
            if attr_str.contains("must_be_true") {
                // 提取消息
                let message = extract_message_from_attr(&attr_str);

                // 重建属性，移除 must_be_true
                let should_remove = if let Some(new_attr_str) = rebuild_attr_without_must_be_true(&attr_str) {
                    // 使用 TokenStream 解析并调用 Attribute::parse_outer
                    let attr_updated = if let Ok(tokens) = proc_macro2::TokenStream::from_str(&new_attr_str) {
                        // 调用 Attribute::parse_outer.parse2
                        if let Ok(attrs) = syn::Attribute::parse_outer.parse2(tokens) {
                            if let Some(new_attr) = attrs.first() {
                                field.attrs[i] = new_attr.clone();
                                true  // 成功更新
                            } else {
                                false  // 没有返回属性
                            }
                        } else {
                            false  // 解析失败
                        }
                    } else {
                        false  // 转换失败
                    };

                    !attr_updated
                } else {
                    // 如果返回 None，说明清理后属性为空，需要移除
                    true
                };

                if should_remove {
                    field.attrs.remove(i);
                }

                return message;
            }
        }
    }

    None
}

/// # 从属性字符串中提取消息
///
/// 从 `#[validate(must_be_true(message = "..."))]` 或类似的属性字符串中提取消息。
///
/// ## 参数
/// - `attr_str`: 属性的字符串表示
///
/// ## 返回
/// 提取的消息字符串，如果没有找到则返回默认消息 "must be true"
fn extract_message_from_attr(attr_str: &str) -> Option<String> {
    // 查找 must_be_true(message = "...")
    if let Some(must_be_true_start) = attr_str.find("must_be_true") {
        let after_must_be_true = &attr_str[must_be_true_start..];

        // 查找 message 参数
        if let Some(message_start) = after_must_be_true.find("message") {
            if let Some(eq_pos) = after_must_be_true[message_start..].find('=') {
                let after_eq = &after_must_be_true[message_start + eq_pos + 1..];

                // 跳过空格
                let after_eq = after_eq.trim_start();

                // 查找引号包裹的字符串
                if after_eq.starts_with('"') {
                    if let Some(end_quote) = after_eq[1..].find('"') {
                        let message = &after_eq[1..end_quote + 1];
                        return Some(message.to_string());
                    }
                }
            }
        } else {
            // 如果没有 message 参数，返回默认消息
            return Some("must be true".to_string());
        }
    }

    // 默认消息
    Some("must be true".to_string())
}

/// # 重建不包含 must_be_true 的属性
///
/// 从 `#[validate(...)]` 属性中移除 `must_be_true(...)` 部分，重建属性。
///
/// ## 参数
/// - `attr_str`: 原始属性的字符串表示
///
/// ## 返回
/// - `Some(String)`: 重建后的属性字符串
/// - `None`: 如果清理后属性为空
fn rebuild_attr_without_must_be_true(attr_str: &str) -> Option<String> {
    // 移除 #[ 和 ]
    let inner = attr_str.trim_start_matches("#[").trim_end_matches("]");

    // 提取 validate(...) 部分
    if inner.starts_with("validate") {
        let validate_start = inner.find('(')?;
        let validate_end = inner.rfind(')')?;
        let validate_content = &inner[validate_start + 1..validate_end];

        // 使用正则表达式移除 must_be_true(...) 部分
        // 模式匹配: must_be_true(message = "...") 或 must_be_true
        let re = regex::Regex::new(r#"must_be_true\s*\([^)]*\)\s*,?"#).unwrap();
        let cleaned = re.replace_all(validate_content, "").to_string();

        // 清理多余的逗号和空格
        let cleaned = cleaned.trim().trim_start_matches(',').trim().trim_end_matches(',').trim();

        // 如果清理后为空，返回 None（移除整个属性）
        if cleaned.is_empty() {
            return None;
        }

        // 重建属性字符串
        Some(format!("#[validate({})]", cleaned))
    } else {
        None
    }
}

/// # 生成验证方法名称
///
/// 根据字段名称生成合适的验证方法名称。
///
/// ## 参数
/// - `field_name`: 字段的标识符
/// - `counter`: 字段计数器的可变引用
///
/// ## 返回
/// 生成的验证方法名称
///
/// ## 命名规则
/// - 包含 "terms" 的字段 → `validate_accept_terms`
/// - 包含 "privacy" 的字段 → `validate_accept_privacy`
/// - 其他字段 → `validate_field_{counter}`
fn generate_validation_fn_name(field_name: &syn::Ident, counter: &mut usize) -> syn::Ident {
    let field_str = field_name.to_string();
    if field_str.contains("terms") {
        syn::Ident::new("validate_accept_terms", proc_macro2::Span::call_site())
    } else if field_str.contains("privacy") {
        syn::Ident::new("validate_accept_privacy", proc_macro2::Span::call_site())
    } else {
        let name = format!("validate_field_{}", counter);
        *counter += 1;
        syn::Ident::new(&name, proc_macro2::Span::call_site())
    }
}

/// # 测试辅助模块
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_message_from_attr() {
        let attr = r#"#[validate(must_be_true(message = "必须同意用户协议"))]"#;
        let message = extract_message_from_attr(attr);
        assert_eq!(message, Some("必须同意用户协议".to_string()));
    }

    #[test]
    fn test_extract_message_without_custom_message() {
        let attr = r#"#[validate(must_be_true)]"#;
        let message = extract_message_from_attr(attr);
        assert_eq!(message, Some("must be true".to_string()));
    }

    #[test]
    fn test_rebuild_attr_without_must_be_true() {
        let attr = r#"#[validate(must_be_true(message = "必须同意"))]"#;
        let result = rebuild_attr_without_must_be_true(attr);
        // 如果移除后为空，应该返回 None
        assert!(result.is_none());
    }

    #[test]
    fn test_rebuild_attr_with_other_validations() {
        let attr = r#"#[validate(length(min = 1, max = 100), must_be_true(message = "必须同意"))]"#;
        let result = rebuild_attr_without_must_be_true(attr);
        assert!(result.is_some());
        if let Some(new_attr) = result {
            assert!(!new_attr.contains("must_be_true"));
            assert!(new_attr.contains("length"));
        }
    }

    #[test]
    fn test_generate_validation_fn_name_terms() {
        let mut counter = 0;
        let ident = syn::Ident::new("accept_terms", proc_macro2::Span::call_site());
        let fn_name = generate_validation_fn_name(&ident, &mut counter);
        assert_eq!(fn_name.to_string(), "validate_accept_terms");
    }

    #[test]
    fn test_generate_validation_fn_name_privacy() {
        let mut counter = 0;
        let ident = syn::Ident::new("accept_privacy", proc_macro2::Span::call_site());
        let fn_name = generate_validation_fn_name(&ident, &mut counter);
        assert_eq!(fn_name.to_string(), "validate_accept_privacy");
    }

    #[test]
    fn test_generate_validation_fn_name_generic() {
        let mut counter = 0;
        let ident = syn::Ident::new("enabled", proc_macro2::Span::call_site());
        let fn_name = generate_validation_fn_name(&ident, &mut counter);
        assert_eq!(fn_name.to_string(), "validate_field_0");
        assert_eq!(counter, 1);
    }
}
