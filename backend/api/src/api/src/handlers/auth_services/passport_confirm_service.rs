use super::{BLServices, ServiceError, BUSINESS_PROCCESS};
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use models::Document;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::paths::PASSPORT_CONF_SERVICE_PATH as PATH;

use super::{ResponseStatusCode, ResponseStatusCodeType, ResponseWithoutData, StatusResponse};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct PassportConfRequest {
    #[schema(example = "exist@exist.com")]
    email: String,
    passport: Document,
}

#[utoipa::path(
    post,
    path = "/user/passport-confirm",
    request_body = PassportConfRequest,
    responses(
        (status = StatusCode::OK, description = "Пользователь успешно обновил паспортные данные", body = ResponseWithoutData),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["auth"]
)]
pub async fn handle_passport_conf(
    ExtractJson(payload): ExtractJson<PassportConfRequest>,
) -> Result<Json<ResponseWithoutData>, StatusCode> {
    let mut status = StatusResponse::new();
    println!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_PROCCESS.get("auther") {
        Some(BLServices::AuthService(s)) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match service.passport_confirm(&payload.email, &payload.passport) {
        Ok(_) => return Ok(Json(ResponseWithoutData { status })),
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                return Ok(Json(ResponseWithoutData { status }));
            }
            ServiceError::NotFoundError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
                status.message = format!("{e} not founded");
                return Ok(Json(ResponseWithoutData { status }));
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };
}
