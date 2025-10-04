use super::VERSION;
use super::{CoreServices, ServiceError, ServicesContainer};
use crate::paths::{vpath, REG_SERVICE_V2_PATH as PATH};
use axum::response::{IntoResponse, Response};
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{ResponseStatusCode, ResponseStatusCodeType, StatusResponse};

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct RegRequest {
    #[schema(example = "firstname")]
    pub firstname: String,
    #[schema(example = "surname")]
    pub surname: String,
    #[schema(example = "lastname")]
    pub lastname: Option<String>,
    #[schema(example = "email@example.com")]
    pub email: String,
    #[schema(example = "password")]
    pub pswd: String,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/v2/users/registr",
    summary = "Регистрация",
    description = "Регистрация нового пользователя",
    request_body = RegRequest,
    responses(
        (status = StatusCode::NO_CONTENT, description = "Пользователь успешно зарегестрирован"),
        (status = StatusCode::CONFLICT, description = "Пользователь с заданным email уже существует", body = StatusResponse),
        (status = StatusCode::BAD_REQUEST, description = "Невалидные данные", body = StatusResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["auth"]
)]
pub async fn handle_reg_v2(ExtractJson(payload): ExtractJson<RegRequest>) -> Response {
    let mut status = StatusResponse::new();
    log::info!(
        "Received request from {}: {:?}",
        vpath(VERSION, PATH.as_str()),
        payload
    );

    let service = match ServicesContainer::get("auther").await {
        Some(CoreServices::AuthService(s)) => s,
        _ => {
            log::warn!("Can't get AuthService");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    match service
        .register_without_pswd_confirm(
            &payload.firstname,
            &payload.surname,
            payload.lastname,
            &payload.email,
            &payload.pswd,
        )
        .await
    {
        Ok(_) => {}
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");

                if e == "password" {
                    status.message = format!("{} (limit: password len >= 8)", status.message)
                }

                log::warn!("{}", status.message);
                return (StatusCode::BAD_REQUEST, Json(status)).into_response();
            }
            ServiceError::IsExistError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::EXIST_DATA) as isize;
                status.message = format!("{e} is exist");

                log::warn!("{}", status.message);
                return (StatusCode::CONFLICT, Json(status)).into_response();
            }
            _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    };

    log::info!("User successfully registered");
    StatusCode::NO_CONTENT.into_response()
}
