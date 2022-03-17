use axum::{response::{IntoResponse, Response}, body::boxed};
use hyper::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AppException {
    #[serde(skip_serializing)]
    code: StatusCode,
    message: String,
}

impl AppException {
    pub fn new(
        code: StatusCode,
        message: String
    ) -> Self { 
        Self { 
            code,
            message 
        } 
    }
}

impl IntoResponse for AppException {
    fn into_response(self) -> Response {
        let body = boxed(axum::body::Full::from(serde_json::to_string(&self).unwrap()));

        Response::builder()
            .header("Content-Type", "application/json")
            .status(self.code)
            .body(body)
            .unwrap()
    }
}
