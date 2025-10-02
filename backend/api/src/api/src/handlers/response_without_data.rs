use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::status_response::StatusResponse;

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct ResponseWithoutData {
    #[schema(example = json!({ "code": 0, "message": "OK" }))]
    pub status: StatusResponse,
}
