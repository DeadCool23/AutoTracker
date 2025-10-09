pub mod car_search_service;
pub mod track_info_search_service;

use super::token_auth::get_auth_data;
use super::StatusResponse;
use super::VERSION;
use super::{CoreServices, ServiceError, ServicesContainer};
use super::{ResponseStatusCode, ResponseStatusCodeType};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

fn validate_pagination(limit: Option<isize>) -> Result<(), Response> {
    if let Some(l) = limit {
        if l < -1 || l == 0 {
            let mut status = StatusResponse::new();
            status.code = ResponseStatusCode::INVALID_LIMIT as isize;
            status.message = "Invalid limit: must be -1 or > 0".to_string();

            log::warn!("{}", status.message);
            return Err((StatusCode::BAD_REQUEST, Json(status)).into_response());
        }
    }
    Ok(())
}

fn handle_search_error(err: ServiceError) -> Response {
    let mut status = StatusResponse::new();

    match err {
        ServiceError::InvalidDataError(e) => {
            status.code =
                ResponseStatusCode::from(&e, ResponseStatusCodeType::INVALID_DATA) as isize;
            status.message = format!("Invalid {e}");
            log::warn!("{}", status.message);
            (StatusCode::BAD_REQUEST, Json(status)).into_response()
        }
        ServiceError::NotFoundError(e) => {
            status.code =
                ResponseStatusCode::from(&e, ResponseStatusCodeType::NOT_FOUNDED_DATA) as isize;
            status.message = format!("Not found: {e}");
            log::warn!("{}", status.message);
            (StatusCode::NOT_FOUND, Json(status)).into_response()
        }
        _ => {
            log::error!("Unexpected internal error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
