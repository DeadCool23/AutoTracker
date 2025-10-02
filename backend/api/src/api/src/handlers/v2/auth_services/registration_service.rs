use super::{CoreServices, ServiceError, ServicesContainer};
use crate::paths::REG_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{ResponseStatusCode, ResponseStatusCodeType, ResponseWithoutData, StatusResponse};

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
    #[schema(example = "password")]
    pub rep_pswd: String,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/api/v2/users/registr",
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

    let service = match ServicesContainer::get("auther").await {
        Some(CoreServices::AuthService(s)) => s,
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
