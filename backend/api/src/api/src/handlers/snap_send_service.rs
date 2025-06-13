use super::ResponseWithoutData;
use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use crate::paths::SNAP_SEND_SERVICE_PATH as PATH;
use axum::{
    extract::Json as ExtractJson,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use models::Camera;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct SnapSendRequest {
    #[schema(example = 70)]
    speed: Option<u16>,
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

#[axum::debug_handler]
#[utoipa::path(
    put,
    path = "/snap/send",
    summary = "Отравление снимка",
    description = "Отравление снимка автомобиля",
    request_body = SnapSendRequest,
    responses(
        (status = StatusCode::CREATED, description = "Снимок успешно добавлен", body = ResponseWithoutData),
        (status = StatusCode::BAD_REQUEST, description = "Некорректные входные параметры", body = ResponseWithoutData),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["snap"]
)]
pub async fn handle_snap_send(
    ExtractJson(payload): ExtractJson<SnapSendRequest>,
) -> Result<Response, StatusCode> {
    let mut status = StatusResponse::new();
    log::info!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_SERVICES::get("snap_sender").await {
        Some(BLServices::SnapSendService(s)) => s,
        _ => {
            log::warn!("Can't get SnapSendService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match service
        .insert_snap(
            &payload.camera,
            payload.speed,
            &payload.time,
            &payload.date,
            &payload.gos_num,
        )
        .await
    {
        Ok(_) => {}
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                let response = ResponseWithoutData { status };
                log::warn!("Sended error response {:#?}", response);

                return Ok((StatusCode::BAD_REQUEST, Json(response)).into_response());
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    let response = ResponseWithoutData { status };
    log::info!("Sended response {:#?}", response);

    Ok((StatusCode::CREATED, Json(response)).into_response())
}
