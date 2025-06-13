use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use crate::paths::PASSPORT_CONF_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use models::Document;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{ResponseStatusCode, ResponseStatusCodeType, ResponseWithoutData, StatusResponse};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct PassportConfRequest {
    #[schema(example = "exist@exist.com")]
    email: String,
    passport: Document,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/user/passport-confirm",
    summary = "Подтверждениее пасспортных данных",
    description = "Подтверждениее пасспортных данных пользователя",
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

    let service = match BUSINESS_SERVICES::get("auther").await {
        Some(BLServices::AuthService(s)) => s,
        _ => {
            log::warn!("Can't get AuthService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let response = match service
        .passport_confirm(&payload.email, &payload.passport)
        .await
    {
        Ok(_) => ResponseWithoutData { status },
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");

                ResponseWithoutData { status }
            }
            ServiceError::IsExistError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::EXIST_DATA) as isize;
                status.message = format!("{e} is exist");

                ResponseWithoutData { status }
            }
            ServiceError::NotFoundError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
                status.message = format!("{e} not founded");

                ResponseWithoutData { status }
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    log::info!("Sended response {:#?}", response);
    Ok(Json(response))
}
