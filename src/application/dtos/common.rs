use serde::{Deserialize, Serialize};

pub const DEFAULT_PAGE_SIZE: i64 = 20;
pub const MAX_PAGE_SIZE: i64 = 50;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl PaginationParams {
    pub fn normalize(&self) -> (i64, i64) {
        let page = self.page.unwrap_or(1).max(1);
        let page_size = self
            .page_size
            .unwrap_or(DEFAULT_PAGE_SIZE)
            .clamp(1, MAX_PAGE_SIZE);
        (page, page_size)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub page_size: i64,
    pub total_items: i64,
    pub total_pages: i64,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    pub success: bool,
    pub code: String,
    pub message: String,
    pub data: Option<T>,
    pub meta: ApiMeta,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorResponse {
    pub success: bool,
    pub code: String,
    pub message: String,
    pub errors: Option<Vec<ApiErrorDetail>>,
    pub meta: ApiMeta,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorDetail {
    pub field: String,
    pub reason: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiMeta {
    pub request_id: String,
    pub timestamp: String,
    pub version: String,
}
