// src/web/pagination.rs
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use utoipa::ToSchema;

/// 分页数据
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedData<T> {
    /// 数据项
    pub items: T,

    /// 当前页码
    pub page: u64,

    /// 每页数量
    pub page_size: u64,

    /// 总记录数
    pub total: u64,

    /// 总页数
    pub total_pages: u64,

    /// 是否有上一页
    pub has_previous: bool,

    /// 是否有下一页
    pub has_next: bool,

    /// 上一页页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<u64>,

    /// 下一页页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<u64>,
}

impl<T> PaginatedData<T> {
    /// 创建分页数据
    pub fn new(items: T, page: u64, page_size: u64, total: u64) -> Self {
        let total_pages = (total as f64 / page_size as f64).ceil() as u64;
        let has_previous = page > 1;
        let has_next = page < total_pages;
        let previous_page = if has_previous { Some(page - 1) } else { None };
        let next_page = if has_next { Some(page + 1) } else { None };
        Self {
            items,
            page,
            page_size,
            total,
            total_pages,
            has_previous,
            has_next,
            previous_page,
            next_page,
        }
    }

    /// 获取当前页数据
    pub fn items(&self) -> &T {
        &self.items
    }

    /// 获取分页信息
    pub fn pagination_info(&self) -> PaginationInfo {
        PaginationInfo {
            page: self.page,
            page_size: self.page_size,
            total: self.total,
            total_pages: self.total_pages,
            has_previous: self.has_previous,
            has_next: self.has_next,
            previous_page: self.previous_page,
            next_page: self.next_page,
        }
    }
}

/// 分页信息
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationInfo {
    /// 当前页码
    pub page: u64,

    /// 每页数量
    pub page_size: u64,

    /// 总记录数
    pub total: u64,

    /// 总页数
    pub total_pages: u64,

    /// 是否有上一页
    pub has_previous: bool,

    /// 是否有下一页
    pub has_next: bool,

    /// 上一页页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_page: Option<u64>,

    /// 下一页页码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<u64>,
}

/// 分页查询参数
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginationParams {
    /// 页码，从1开始
    #[serde(default = "default_page")]
    pub page: u64,

    /// 每页数量
    #[serde(default = "default_page_size")]
    pub page_size: u64,

    /// 排序字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,

    /// 排序方向
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<SortDirection>,

    /// 搜索关键字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,

    /// 筛选条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<serde_json::Value>,
}

fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
            sort_by: None,
            sort_direction: None,
            search: None,
            filters: None,
        }
    }
}

impl PaginationParams {
    /// 创建分页参数
    pub fn new(page: u64, page_size: u64) -> Self {
        Self {
            page,
            page_size,
            ..Default::default()
        }
    }

    /// 获取偏移量
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.page_size
    }

    /// 获取限制
    pub fn limit(&self) -> u64 {
        self.page_size
    }

    /// 验证分页参数
    pub fn validate(&self) -> Result<(), String> {
        if self.page < 1 {
            return Err("页码必须大于0".to_string());
        }

        if self.page_size < 1 || self.page_size > 100 {
            return Err("每页数量必须在1-100之间".to_string());
        }

        Ok(())
    }

    /// 设置排序
    pub fn with_sort(mut self, sort_by: &str, direction: SortDirection) -> Self {
        self.sort_by = Some(sort_by.to_string());
        self.sort_direction = Some(direction);
        self
    }

    /// 设置搜索
    pub fn with_search(mut self, search: &str) -> Self {
        self.search = Some(search.to_string());
        self
    }

    /// 设置筛选
    pub fn with_filters(mut self, filters: serde_json::Value) -> Self {
        self.filters = Some(filters);
        self
    }
}

/// 排序方向
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum SortDirection {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::Desc
    }
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDirection::Asc => write!(f, "asc"),
            SortDirection::Desc => write!(f, "desc"),
        }
    }
}
