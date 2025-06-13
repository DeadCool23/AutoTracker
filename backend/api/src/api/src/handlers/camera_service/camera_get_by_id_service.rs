use super::camera_response::CameraResponse;
use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};
use crate::paths::CAMERA_GET_BY_ID_SERVICE_PATH as PATH;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[axum::debug_handler]
#[utoipa::path(
    get,
    path = "/camera/{id}",
    summary = "Получение камеры",
    description = "Получение камеры по id",
    responses(
        (status = StatusCode::OK, description = "Камера успешно получена", body = CameraResponse),
        (status = StatusCode::NOT_FOUND, description = "Камера не найдена", body = CameraResponse, example = json!({
            "status": {
                "code": 4002,
                "message": "camera not found"
            },
            "camera": null
        })),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["camera"]
)]
pub async fn handle_get_camera_by_id(Path(id): Path<i64>) -> Result<Response, StatusCode> {
    let mut status = StatusResponse::new();
    log::info!(
        "Received request from {}: {{ id: {:?} }}",
        PATH.as_str(),
        id
    );

    let service = match BUSINESS_SERVICES::get("camera_data_getter").await {
        Some(BLServices::CameraDataGetService(s)) => s,
        _ => {
            log::warn!("Can't get CameraDataGetService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let camera = match service.get_camera_by_id(id as usize).await {
        Ok(camera) => camera,
        Err(e) => match e {
            ServiceError::NotFoundError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
                status.message = format!("{e} not found");
                let response = CameraResponse {
                    status,
                    camera: None,
                };
                return Ok((StatusCode::NOT_FOUND, Json(response)).into_response());
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    let response = CameraResponse {
        status,
        camera: Some(camera),
    };
    log::info!("Sended response {:#?}", response);

    Ok((StatusCode::OK, Json(response)).into_response())
}
