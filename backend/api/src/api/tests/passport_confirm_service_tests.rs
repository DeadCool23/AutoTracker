use axum::{routing::post, Router};
use axum_test::{http::StatusCode, TestServer};
use serde_json::json;

use api::handlers::{
    auth_services::passport_confirm_service::handle_passport_conf,
    response_status_code::ResponseStatusCode, ResponseWithoutData,
};
use api::paths;

#[tokio::test]
async fn test_handle_passport_conf_success() {
    let app = Router::new().route(
        &paths::PASSPORT_CONF_SERVICE_PATH,
        post(handle_passport_conf),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::PASSPORT_CONF_SERVICE_PATH)
        .json(&json!({
            "email": "exist@exist.com",
            "passport": {
                "serial": "1111",
                "number": "111111"
            }
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}

#[tokio::test]
async fn test_handle_passport_conf_invalid_email() {
    let app = axum::Router::new().route(
        &paths::PASSPORT_CONF_SERVICE_PATH,
        post(handle_passport_conf),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::PASSPORT_CONF_SERVICE_PATH)
        .json(&json!({
            "email": "existexist.com",
            "passport": {
                "serial": "1111",
                "number": "111111"
            }
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::INVALID_EMAIL as isize);
}

#[tokio::test]
async fn test_handle_passport_conf_invalid_passport_serial() {
    let app = axum::Router::new().route(
        &paths::PASSPORT_CONF_SERVICE_PATH,
        post(handle_passport_conf),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::PASSPORT_CONF_SERVICE_PATH)
        .json(&json!({
            "email": "exist@exist.com",
            "passport": {
                "serial": "111",
                "number": "111111"
            }
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_PASSPORT as isize
    );
}

#[tokio::test]
async fn test_handle_passport_conf_invalid_passport_number() {
    let app = axum::Router::new().route(
        &paths::PASSPORT_CONF_SERVICE_PATH,
        post(handle_passport_conf),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::PASSPORT_CONF_SERVICE_PATH)
        .json(&json!({
            "email": "exist@exist.com",
            "passport": {
                "serial": "1111",
                "number": "1111111"
            }
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::INVALID_PASSPORT as isize
    );
}

#[tokio::test]
async fn test_handle_passport_conf_email_not_founded() {
    let app = axum::Router::new().route(
        &paths::PASSPORT_CONF_SERVICE_PATH,
        post(handle_passport_conf),
    );
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::PASSPORT_CONF_SERVICE_PATH)
        .json(&json!({
            "email": "email@example.com",
            "passport": {
                "serial": "1111",
                "number": "111111"
            }
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(
        body.status.code,
        ResponseStatusCode::EMAIL_NOT_FOUNDED as isize
    );
}
