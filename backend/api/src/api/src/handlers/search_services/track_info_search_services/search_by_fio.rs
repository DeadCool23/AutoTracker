use super::{BLServices, BUSINESS_SERVICES};
use super::{SearchByFIORequest, TrackInfoSearcherResponse};
use crate::paths::TRACK_INFO_SEARCH_BY_FIO_SERVICE_PATH as PATH;
use axum::{extract::Json as ExtractJson, http::StatusCode, Json};

use super::StatusResponse;

#[utoipa::path(
    post,
    path = "/track-info/search/by-fio",
    summary = "Поиск отслеживаний",
    description = "Поиск отслеживаний по ФИО пользователя",
    request_body = SearchByFIORequest,
    responses(
        (status = StatusCode::OK, description = "Информация об отслеживании успешно найдена", body = TrackInfoSearcherResponse),
        (status = StatusCode::INTERNAL_SERVER_ERROR, description = "Внутренняя ошибка сервера"),
    ),
    tags = ["search", "track-info"]
)]
pub async fn handle_search_track_info_by_fio(
    ExtractJson(payload): ExtractJson<SearchByFIORequest>,
) -> Result<Json<TrackInfoSearcherResponse>, StatusCode> {
    let status = StatusResponse::new();
    log::info!("Received request from {}: {:?}", PATH.as_str(), payload);

    let service = match BUSINESS_SERVICES::get("searcher").await {
        Some(BLServices::SearchService(s)) => s,
        _ => {
            log::warn!("Can't get SearchService");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let response = match service
        .search_track_info_by_owner_fio(payload.name, payload.surname, payload.lastname)
        .await
    {
        Ok(track_info) => TrackInfoSearcherResponse { status, track_info },
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    log::info!("Sended response {:#?}", response);
    Ok(Json(response))
}
