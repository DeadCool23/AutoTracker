use business_logic::services::auth_service::AuthService;
use business_logic::services_traits::Authorizer;
use data_access::repositories::mocked::MockUserRepo;

#[tokio::test]
async fn test_handle_reg_success() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .register(
            &"firstname".to_string(),
            &"surname".to_string(),
            Some("lastname".to_string()),
            &"email@example.com".to_string(),
            &"password".to_string(),
            &"password".to_string(),
        )
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_reg_invalid_email() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .register(
            &"firstname".to_string(),
            &"surname".to_string(),
            Some("lastname".to_string()),
            &"emailexample.com".to_string(),
            &"password".to_string(),
            &"password".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: email");
}

#[tokio::test]
async fn test_handle_reg_invalid_pswd() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .register(
            &"firstname".to_string(),
            &"surname".to_string(),
            Some("lastname".to_string()),
            &"email@example.com".to_string(),
            &"pass".to_string(),
            &"pass".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: password");
}

#[tokio::test]
async fn test_handle_reg_invalid_pswds() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .register(
            &"firstname".to_string(),
            &"surname".to_string(),
            Some("lastname".to_string()),
            &"email@example.com".to_string(),
            &"password".to_string(),
            &"password1".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(
        res.err().unwrap().to_string(),
        "Invalid data: passwords unmatch"
    );
}

#[tokio::test]
async fn test_handle_reg_email_exist() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .register(
            &"firstname".to_string(),
            &"surname".to_string(),
            Some("lastname".to_string()),
            &"exist@exist.com".to_string(),
            &"password".to_string(),
            &"password".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "email already exist");
}
