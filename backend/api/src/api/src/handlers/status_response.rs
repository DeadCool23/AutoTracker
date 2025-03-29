use super::ResponseStatusCode;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct StatusResponse {
    #[schema(example = 0)]
    pub code: isize,
    #[schema(example = "OK")]
    pub message: String,
}

#[allow(dead_code)]
impl StatusResponse {
    pub fn new() -> Self {
        StatusResponse {
            code: ResponseStatusCode::OK as isize,
            message: "OK".to_string(),
        }
    }

    fn from(code: ResponseStatusCode, message: String) -> Self {
        StatusResponse {
            code: code as isize,
            message,
        }
    }
}
