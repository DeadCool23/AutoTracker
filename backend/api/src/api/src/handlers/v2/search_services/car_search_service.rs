use super::get_auth_data;
use super::VERSION;
use super::{CoreServices, ServiceError, ServicesContainer};
use super::{ResponseStatusCode, ResponseStatusCodeType};
use crate::paths::{vpath, CAR_SEARCH_SERVICE_PATH as PATH};
use axum::response::{IntoResponse, Response};
use axum::{
    extract::Json as ExtractJson,
    http::{HeaderMap, StatusCode},
    Json,
};
use jwt_processing::Claims;
use models::{Car, Document, Role};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::StatusResponse;

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct SearchCarsRequest {
    #[schema(example = "name")]
    pub name: Option<String>,
    #[schema(example = "surname")]
    pub surname: Option<String>,
    #[schema(example = "lastname")]
    pub lastname: Option<String>,
    #[schema(value_type = Document)]
    pub passport: Option<Document>,
    #[schema(example = "А*23**99")]
    pub gos_num: Option<String>,
    #[schema(example = 0)]
    pub offset: Option<usize>,
    #[schema(example = 100)]
    pub limit: Option<isize>,
}

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct CarSearcherResponse {
    #[schema(example = 0)]
    pub offset: usize,
    #[schema(example = json!([]))]
    pub cars: Vec<Car>,
}

fn check_roots(claim: &Claims, request: &SearchCarsRequest) -> Result<(), StatusCode> {
    if claim.role == Role::audit
        || (claim.role == Role::user && request.passport.is_none())
        || (claim.role == Role::user && request.passport != claim.passport)
    {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/v2/cars/search",
    summary = "Поиск автомобилей",
    description = "Поиск автомобилей по общим фильтрам",
    request_body = SearchCarsRequest,
    security(
        ("jwt_bearer_auth" = [])
    ),
    responses(
        (status = StatusCode::OK, description = "Автомобили успешно найдены", body = CarSearcherResponse),
        (status = StatusCode::UNAUTHORIZED, description = "Пользователь не авторизирован"),
        (status = StatusCode::FORBIDDEN, description = "Недостаточно прав"),
        (status = StatusCode::BAD_REQUEST, description = "Невалидные данные", body = StatusResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "car"]
)]
pub async fn handle_search_cars_by_filters_with_offset_v2(
    headers: HeaderMap,
    ExtractJson(payload): ExtractJson<SearchCarsRequest>,
) -> Response {
    let mut status = StatusResponse::new();
    log::info!(
        "Received request from {}: {:?}",
        vpath(VERSION, PATH.as_str()),
        payload
    );

    let claim = match get_auth_data(headers) {
        Ok(c) => c,
        Err(code) => return code.into_response(),
    };

    if let Err(code) = check_roots(&claim, &payload) {
        return code.into_response();
    }

    let service = match ServicesContainer::get("searcher").await {
        Some(CoreServices::SearchService(s)) => s,
        _ => {
            log::warn!("Can't get SearchService");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let offset = payload.offset.unwrap_or(0);
    let limit = payload.limit.unwrap_or(-1);
    if limit < -1 || limit == 0 {
        status.code = ResponseStatusCode::INVALID_LIMIT as isize;
        status.message = "Invalid limit: limit == -1 || limit > 0".to_string();

        log::warn!("{}", status.message);
        return (StatusCode::BAD_REQUEST, Json(status)).into_response();
    }

    let response = match service
        .search_car_with_offset(
            payload.name,
            payload.surname,
            payload.lastname,
            payload.passport,
            payload.gos_num,
            offset,
            limit,
        )
        .await
    {
        Ok(cars) => {
            let new_offset = offset + cars.len();
            CarSearcherResponse {
                cars,
                offset: new_offset,
            }
        }
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");

                log::warn!("{}", status.message);
                return (StatusCode::BAD_REQUEST, Json(status)).into_response();
            }
            _ => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        },
    };

    log::info!("Sended response {:#?}", response);
    Json(response).into_response()
}
