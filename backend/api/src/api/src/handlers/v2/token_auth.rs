use axum::http::{HeaderMap, StatusCode};
use jwt_processing::{verify_jwt, Claims};

pub fn get_auth_data(headers: HeaderMap) -> Result<Claims, StatusCode> {
    let auth_header = headers
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    log::info!("Auth token: {}", auth_header);

    let claims = verify_jwt(token).map_err(|e| {
        log::error!("{}", e);
        StatusCode::UNAUTHORIZED
    })?;

    Ok(claims)
}
