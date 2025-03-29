use super::ResponseWithoutData;
use super::{BLServices, ServiceError, BUSINESS_PROCCESS};
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use models::Camera;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::paths::SNAP_SEND_SERVICE_PATH as PATH;

use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct SnapSendRequest {
    #[schema(example = "8:10")]
    time: String,
    #[schema(example = "01.01.2025")]
    date: String,
    #[schema(example = "А777МР77")]
    gos_num: String,
    #[schema(example = json!(
        "id": 1,
        "location": { 
            "longitude": 54.98989, 
            "latitude": 56.89882 
        }
    ))]
    camera: Camera,
}

#[utoipa::path(
    put,
    path = "/snap/send",
    request_body = SnapSendRequest,
    responses(
        (status = StatusCode::OK, description = "Снимок успешно добавлен", body = ResponseWithoutData),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["snap"]
)]
pub async fn handle_snap_send(
    ExtractJson(payload): ExtractJson<SnapSendRequest>,
) -> Result<Json<ResponseWithoutData>, StatusCode> {
    let mut status = StatusResponse::new();
    println!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_PROCCESS.get("snap_sender") {
        Some(BLServices::SnapSendService(s)) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match service.insert_snap(
        &payload.camera,
        &payload.time,
        &payload.date,
        &payload.gos_num,
    ) {
        Ok(_) => {}
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                return Ok(Json(ResponseWithoutData { status }));
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    let response = ResponseWithoutData { status };

    Ok(Json(response))
}
