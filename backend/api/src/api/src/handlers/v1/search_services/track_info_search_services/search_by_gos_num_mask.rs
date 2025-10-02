use super::{CoreServices, ServiceError, ServicesContainer};
use super::{ResponseStatusCode, ResponseStatusCodeType};
use super::{SearchByGosNumRequest, TrackInfoSearcherResponse};

use super::VERSION;
use crate::paths::{vpath, TRACK_INFO_SEARCH_BY_GOS_NUM_MASK_SERVICE_PATH as PATH};

use axum::{extract::Json as ExtractJson, http::StatusCode, Json};

use super::StatusResponse;

#[utoipa::path(
    post,
    path = "/api/v1/track-info/search/by-gos-num-mask",
    summary = "Поиск отслеживаний",
    description = "Поиск отслеживаний по маске гос.номера",
    request_body = SearchByGosNumRequest,
    responses(
        (status = StatusCode::OK, description = "Информация об отслеживании успешно найдена", body = TrackInfoSearcherResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "track-info"]
)]
pub async fn handle_search_track_info_by_gos_num_mask(
    ExtractJson(payload): ExtractJson<SearchByGosNumRequest>,
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
        .search_track_info_by_gos_num_mask(&payload.gos_num)
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
