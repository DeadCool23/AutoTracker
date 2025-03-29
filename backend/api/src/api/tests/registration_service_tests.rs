use axum::{routing::post, Router};
use axum_test::{http::StatusCode, TestServer};
use serde_json::json;

use api::handlers::{
    auth_services::registration_service::handle_reg, response_status_code::ResponseStatusCode,
    ResponseWithoutData,
};
use api::paths;

#[tokio::test]
async fn test_handle_reg_success() {
    let app = Router::new().route(&paths::REG_SERVICE_PATH, post(handle_reg));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::REG_SERVICE_PATH)
        .json(&json!({
            "email": "email@example.com",
            "firstname": "firstname",
            "lastname": "lastname",
            "pswd": "password",
            "rep_pswd": "password",
            "surname": "surname"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::OK as isize);
}

#[tokio::test]
async fn test_handle_reg_invalid_email() {
    let app = axum::Router::new().route(&paths::REG_SERVICE_PATH, post(handle_reg));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::REG_SERVICE_PATH)
        .json(&json!({
            "email": "emailexample.com",
            "firstname": "firstname",
            "lastname": "lastname",
            "pswd": "password",
            "rep_pswd": "password",
            "surname": "surname"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::INVALID_EMAIL as isize);
}

#[tokio::test]
async fn test_handle_reg_invalid_pswd() {
    let app = axum::Router::new().route(&paths::REG_SERVICE_PATH, post(handle_reg));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::REG_SERVICE_PATH)
        .json(&json!({
            "email": "email@example.com",
            "firstname": "firstname",
            "lastname": "lastname",
            "pswd": "pass",
            "rep_pswd": "password",
            "surname": "surname"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::INVALID_PSWD as isize);
}

#[tokio::test]
async fn test_handle_reg_invalid_pswds() {
    let app = axum::Router::new().route(&paths::REG_SERVICE_PATH, post(handle_reg));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::REG_SERVICE_PATH)
        .json(&json!({
            "email": "email@example.com",
            "firstname": "firstname",
            "lastname": "lastname",
            "pswd": "password",
            "rep_pswd": "password1",
            "surname": "surname"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::INVALID_PSWDS as isize);
}

#[tokio::test]
async fn test_handle_reg_email_exist() {
    let app = axum::Router::new().route(&paths::REG_SERVICE_PATH, post(handle_reg));
    let server = TestServer::new(app).unwrap();

    let response = server
        .post(&paths::REG_SERVICE_PATH)
        .json(&json!({
            "email": "exist@exist.com",
            "firstname": "firstname",
            "lastname": "lastname",
            "pswd": "password",
            "rep_pswd": "password",
            "surname": "surname"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: ResponseWithoutData = response.json();
    assert_eq!(body.status.code, ResponseStatusCode::EMAIL_EXIST as isize);
}
