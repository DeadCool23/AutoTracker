use super::{get_auth_data, validate_pagination, handle_search_error};
use super::VERSION;
use super::{CoreServices, ServicesContainer};
use crate::paths::{vpath, TRACK_INFO_SEARCH_SERVICE_PATH as PATH};
use axum::response::{IntoResponse, Response};
use axum::{
    extract::Json as ExtractJson,
    http::{HeaderMap, StatusCode},
    Json,
};
use jwt_processing::Claims;
use models::{Document, Role, TrackInfo};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::StatusResponse;

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct SearchTrackInfoByFilterRequest {
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
    #[schema(example = "01.01.2025")]
    pub date: Option<String>,
    #[schema(example = 0)]
    pub offset: Option<usize>,
    #[schema(example = 100)]
    pub limit: Option<isize>,
}

#[derive(ToSchema, Deserialize, Serialize, Debug)]
pub struct TrackInfoSearcherResponse {
    #[schema(example = 0)]
    pub offset: usize,
    #[schema(example = json!([]))]
    pub track_infos: Vec<TrackInfo>,
}

fn check_roots(claim: &Claims) -> Result<(), StatusCode> {
    if claim.role != Role::audit {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(())
}

#[utoipa::path(
    post,
    path = "/api/v2/track-infos/search",
    summary = "Поиск отслеживаний",
    description = "Поиск отслеживаний по общим фильтрам",
    request_body = SearchTrackInfoByFilterRequest,
    security(
        ("jwt_bearer_auth" = [])
    ),
    responses(
        (status = StatusCode::OK, description = "Информация об отслеживаниях успешно найдена", body = TrackInfoSearcherResponse),
        (status = StatusCode::UNAUTHORIZED, description = "Пользователь не авторизирован"),
        (status = StatusCode::FORBIDDEN, description = "Недостаточно прав"),
        (status = StatusCode::BAD_REQUEST, description = "Невалидные данные", body = StatusResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "track-info"]
)]
pub async fn handle_search_track_info_by_filters_with_offset_v2(
    headers: HeaderMap,
    ExtractJson(payload): ExtractJson<SearchTrackInfoByFilterRequest>,
) -> Response {
    log::info!(
        "Received request from {}: {:?}",
        vpath(VERSION, PATH.as_str()),
        payload
    );

    let claim = match get_auth_data(headers) {
        Ok(c) => c,
        Err(code) => return code.into_response(),
    };

    if let Err(code) = check_roots(&claim) {
        return code.into_response();
    }

    let service = match ServicesContainer::get("searcher").await {
        Some(CoreServices::SearchService(s)) => s,
        _ => {
            log::warn!("Can't get SearchService");
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if let Err(resp) = validate_pagination(payload.limit) {
        return resp;
    }

    let offset = payload.offset.unwrap_or(0);
    let limit = payload.limit.unwrap_or(-1);

    match service
        .search_track_info_with_offset(
            payload.name,
            payload.surname,
            payload.lastname,
            payload.passport,
            payload.gos_num,
            payload.date,
            offset,
            limit,
        )
        .await
    {
        Ok(track_infos) => {
            let new_offset = offset + track_infos.len();
            let response = TrackInfoSearcherResponse {
                track_infos,
                offset: new_offset,
            };
            log::info!("Sending response {:#?}", response);
            Json(response).into_response()
        },
        Err(e) => handle_search_error(e),
    }
}
