use business_logic::services::auth_service::AuthService;
use business_logic::services_traits::Authorizer;
use data_access::repositories::mocked::MockUserRepo;
use models::Document;

#[tokio::test]
async fn test_handle_passport_conf_success() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .passport_confirm(
            &"exist@exist.com".to_string(),
            &Document {
                serial: "1111".to_string(),
                number: "111111".to_string(),
            },
        )
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_passport_conf_invalid_email() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .passport_confirm(
            &"emailexample.com".to_string(),
            &Document {
                serial: "1111".to_string(),
                number: "111111".to_string(),
            },
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: email");
}

#[tokio::test]
async fn test_handle_passport_conf_invalid_passport_serial() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .passport_confirm(
            &"email@example.com".to_string(),
            &Document {
                serial: "111".to_string(),
                number: "111111".to_string(),
            },
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: passport");
}

#[tokio::test]
async fn test_handle_passport_conf_invalid_passport_number() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .passport_confirm(
            &"email@example.com".to_string(),
            &Document {
                serial: "1111".to_string(),
                number: "1111111".to_string(),
            },
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: passport");
}

#[tokio::test]
async fn test_handle_passport_conf_email_not_founded() {
    let service = AuthService::from(Box::new(MockUserRepo));

    let res = service
        .passport_confirm(
            &"notexist@exist.com".to_string(),
            &Document {
                serial: "1111".to_string(),
                number: "111111".to_string(),
            },
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "email is not found");
}
