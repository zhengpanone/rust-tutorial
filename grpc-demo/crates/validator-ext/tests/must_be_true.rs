//! # 测试 must_be_true_validator 属性宏
//!
//! 这个测试文件验证属性宏的基本功能

use validator::Validate;
use validator_ext::must_be_true_validator;

// 测试用例1：简单的 must_be_true 验证
#[must_be_true_validator]
#[derive(Debug, Clone, Validate)]
struct SimpleTest {
    #[validate(must_be_true(message = "必须同意用户协议"))]
    accept_terms: bool,
}

#[test]
fn test_simple_validation() {
    let test = SimpleTest { accept_terms: true };
    assert!(test.validate_accept_terms().is_ok());

    let test = SimpleTest {
        accept_terms: false,
    };
    assert!(test.validate_accept_terms().is_err());
}

// 测试用例2：混合其他验证规则
#[must_be_true_validator]
#[derive(Debug, Clone, Validate)]
struct MixedValidationTest {
    #[validate(length(min = 3, max = 50))]
    username: String,
    #[validate(must_be_true(message = "必须同意用户协议"))]
    accept_terms: bool,
}

#[test]
fn test_mixed_validation() {
    let test = MixedValidationTest {
        username: "testuser".to_string(),
        accept_terms: true,
    };
    assert!(test.validate_accept_terms().is_ok());
    assert!(test.validate().is_ok());
}
