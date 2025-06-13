use business_logic::services::auth_service::AuthService;
use business_logic::services_traits::Authorizer;
use data_access::repositories::mocked::MockUserRepo;

#[tokio::test]
async fn test_handle_auth_success() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .auth(&"email@example.com".to_string(), &"password".to_string())
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_auth_invalid_email() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .auth(&"not_email".to_string(), &"password".to_string())
        .await;

    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        "Invalid data: email or password"
    );
}

#[tokio::test]
async fn test_handle_auth_invalid_pswd() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .auth(&"email@example.com".to_string(), &"pass".to_string())
        .await;

    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        "Invalid data: email or password"
    );
}
