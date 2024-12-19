use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub data: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

#[derive(Serialize)]
pub struct Pagination {
    pub page: u32,
    pub per_page: u32,
    pub total: u32,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Json<ApiResponse<T>> {
        Json(Self {
            ok: true,
            data,
            pagination: None,
        })
    }

    pub fn success_with_pagination(data: T, pagination: Pagination) -> Json<ApiResponse<T>> {
        Json(Self {
            ok: true,
            data,
            pagination: Some(pagination),
        })
    }
}
