use super::StatusResponse;
use models::Camera;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct CameraResponse {
    pub status: StatusResponse,
    #[schema(value_type = Camera)]
    pub camera: Option<Camera>,
}
