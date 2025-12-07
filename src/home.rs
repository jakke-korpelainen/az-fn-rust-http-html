use axum::response::IntoResponse;

use crate::templates::{self, HtmlTemplate};

pub async fn home_route() -> impl IntoResponse {
    HtmlTemplate(templates::HomeTemplate {})
}
