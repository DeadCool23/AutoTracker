use super::{BLServices, ServiceError, BUSINESS_PROCCESS};
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::paths::REG_SERVICE_PATH as PATH;

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

#[utoipa::path(
    post,
    path = "/user/registr",
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
    println!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_PROCCESS.get("auther") {
        Some(BLServices::AuthService(s)) => s,
        _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    match service.register(
        &payload.firstname,
        &payload.surname,
        &payload.lastname,
        &payload.email,
        &payload.pswd,
        &payload.rep_pswd,
    ) {
        Ok(_) => return Ok(Json(ResponseWithoutData { status })),
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                return Ok(Json(ResponseWithoutData { status }));
            }
            ServiceError::IsExistError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::EXIST_DATA) as isize;
                status.message = format!("{e} is exist");
                return Ok(Json(ResponseWithoutData { status }));
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };
}
