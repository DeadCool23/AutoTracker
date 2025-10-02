use super::TrackInfoSearcherResponse;
use super::VERSION;
use super::{CoreServices, ServiceError, ServicesContainer};
use super::{ResponseStatusCode, ResponseStatusCodeType};
use crate::paths::{vpath, TRACK_INFO_SEARCH_SERVICE_PATH as PATH};
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};
use models::Document;
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
}

#[utoipa::path(
    post,
    path = "/api/v1/track-info/search",
    summary = "Поиск отслеживаний",
    description = "Поиск отслеживаний по общим фильтрам",
    request_body = SearchTrackInfoByFilterRequest,
    responses(
        (status = StatusCode::OK, description = "Информация об отслеживании успешно найдена", body = TrackInfoSearcherResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "track-info"]
)]
pub async fn handle_search_track_info_by_filters(
    ExtractJson(payload): ExtractJson<SearchTrackInfoByFilterRequest>,
) -> Result<Json<TrackInfoSearcherResponse>, StatusCode> {
    let mut status = StatusResponse::new();
    log::info!(
        "Received request from {}: {:?}",
        vpath(VERSION, PATH.as_str()),
        payload
    );

    let service = match ServicesContainer::get("searcher").await {
        Some(CoreServices::SearchService(s)) => s,
        _ => {
            log::warn!("Can't get SearchService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let response = match service
        .search_track_info(
            payload.name,
            payload.surname,
            payload.lastname,
            payload.passport,
            payload.gos_num,
            payload.date,
        )
        .await
    {
        Ok(track_info) => TrackInfoSearcherResponse { status, track_info },
        Err(e) => match e {
            ServiceError::InvalidDataError(e) => {
                status.code =
                    ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
                status.message = format!("Invalid {e}");
                TrackInfoSearcherResponse {
                    status,
                    track_info: vec![],
                }
            }
            _ => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    log::info!("Sended response {:#?}", response);
    Ok(Json(response))
}
