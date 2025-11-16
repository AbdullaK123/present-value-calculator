use serde::{Deserialize, Serialize};
use axum::response::IntoResponse;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PresentValueResponse {
    pub result: f64
}

impl IntoResponse for PresentValueResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
