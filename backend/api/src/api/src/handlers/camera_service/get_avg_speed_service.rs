use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};
use crate::paths::GET_AVG_SPEED_ON_CAMERA_PATH as PATH;
use axum::{
    extract::Json as ExtractJson,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use models::Location;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct AvgSpeedRequest {
    #[schema(example = "О777ОО77")]
    pub gos_num: String,
    #[schema(value_type = Location)]
    pub location: Location,
}

#[derive(ToSchema, Debug, Deserialize, Serialize)]
pub struct AvgSpeedResponse {
    pub status: StatusResponse,
    #[schema(example = 70)]
    pub avg_speed: Option<f64>,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    request_body = AvgSpeedRequest,
    path = "/camera/avg-speed",
    summary = "Получение средней скорости",
    description = "Получение средней скорости автомобиля на конкретной камере",
    responses(
        (status = StatusCode::OK, description = "Средняя скорость успешно получена", body = AvgSpeedResponse),
        (status = StatusCode::NOT_FOUND, description = "Средняя скорость не найдена", body = AvgSpeedResponse, example = json!({
            "status": {
                "code": 4003,
                "message": "average speed not found"
            },
            "avg_speed": null
        })),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["camera"]
)]
pub async fn handle_get_avg_speed_for_car_on_camera(
    ExtractJson(payload): ExtractJson<AvgSpeedRequest>,
) -> Result<Response, StatusCode> {
    let mut status = StatusResponse::new();
    log::info!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_SERVICES::get("camera_data_getter").await {
        Some(BLServices::CameraDataGetService(s)) => s,
        _ => {
            log::warn!("Can't get CameraDataGetService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let avg_speed = match service
        .get_avg_speed_of_car_on_camera_by_gos_num(&payload.gos_num, &payload.location)
        .await
    {
        Ok(avg_speed) => avg_speed,
        Err(e) => match e {
            ServiceError::NotFoundError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
                status.message = format!("{e} not found");
                let response = AvgSpeedResponse {
                    status,
                    avg_speed: None,
                };
                return Ok((StatusCode::NOT_FOUND, Json(response)).into_response());
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    let response = AvgSpeedResponse {
        status,
        avg_speed: Some(avg_speed),
    };
    log::info!("Sended response {:#?}", response);

    Ok((StatusCode::OK, Json(response)).into_response())
}
