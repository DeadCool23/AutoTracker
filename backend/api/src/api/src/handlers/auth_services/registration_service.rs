use super::{BLServices, ServiceError, BUSINESS_SERVICES};
use crate::paths::REG_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{ResponseStatusCode, ResponseStatusCodeType, ResponseWithoutData, StatusResponse};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct RegRequest {
    #[schema(example = "firstname")]
    firstname: String,
    #[schema(example = "surname")]
    surname: String,
    #[schema(example = "lastname")]
    lastname: Option<String>,
    #[schema(example = "email@example.com")]
    email: String,
    #[schema(example = "password")]
    pswd: String,
    #[schema(example = "password")]
    rep_pswd: String,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/user/registr",
    summary = "Регистрация",
    description = "Регистрация нового пользователя",
    request_body = RegRequest,
    responses(
        (status = StatusCode::OK, description = "Пользователь успешно зарегестрирован", body = ResponseWithoutData),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["auth"]
)]
pub async fn handle_reg(
    ExtractJson(payload): ExtractJson<RegRequest>,
) -> Result<Json<ResponseWithoutData>, StatusCode> {
    let mut status = StatusResponse::new();
    log::info!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_SERVICES::get("auther").await {
        Some(BLServices::AuthService(s)) => s,
        _ => {
            log::warn!("Can't get AuthService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let response = match service
        .register(
            &payload.firstname,
            &payload.surname,
            payload.lastname,
            &payload.email,
            &payload.pswd,
            &payload.rep_pswd,
        )
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
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    log::info!("Sended response {:#?}", response);
    Ok(Json(response))
}
