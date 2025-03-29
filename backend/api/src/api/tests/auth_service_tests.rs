use axum::{routing::post, Router};
use axum_test::{http::StatusCode, TestServer};
use serde_json::json;

use api::handlers::{
    auth_services::auth_service::{handle_auth, AuthResponse},
    response_status_code::ResponseStatusCode,
};
use api::paths;

#[tokio::test]
async fn test_handle_auth_success() {
    let app = Router::new().route(&paths::AUTH_SERVICE_PATH, post(handle_auth));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::AUTH_SERVICE_PATH)
        .json(&json!({
            "email": "email@example.com",
            "pswd": "password"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: AuthResponse = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
    assert!(body.user.is_some());
}

#[tokio::test]
async fn test_handle_auth_invalid_email() {
    let app = axum::Router::new().route(&paths::AUTH_SERVICE_PATH, post(handle_auth));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::AUTH_SERVICE_PATH)
        .json(&json!({
            "email": "notemail",
            "pswd": "password"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: AuthResponse = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_AUTH_DATA as isize
    );
}

#[tokio::test]
async fn test_handle_auth_invalid_pswd() {
    let app = axum::Router::new().route(&paths::AUTH_SERVICE_PATH, post(handle_auth));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::AUTH_SERVICE_PATH)
        .json(&json!({
            "email": "email@example.com",
            "pswd": "pass"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: AuthResponse = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_AUTH_DATA as isize
    );
}
