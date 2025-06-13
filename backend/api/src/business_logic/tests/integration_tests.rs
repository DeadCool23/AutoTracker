use business_logic::services::auth_service::AuthService;
use business_logic::services_traits::Authorizer;
use data_access::repositories::postgres::{PgUserRepo, PG_URL};
use models::Document;

#[tokio::test]
async fn test_psql_auth_success() {
    let service = AuthService::from(Box::new(PgUserRepo::from(&PG_URL).await.unwrap()));

    let res = service
        .auth(
            &"nisuev04@mail.ru".to_string(),
            &"12345678".to_string(),
        )
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_psql_auth_invalid_email() {
    let service = AuthService::from(Box::new(PgUserRepo::from(&PG_URL).await.unwrap()));

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
async fn test_psql_change_passport() {
    let service = AuthService::from(Box::new(PgUserRepo::from(&PG_URL).await.unwrap()));

    let res = service
        .passport_confirm(
            &"qwgzenvrwy@outlook.com".to_string(),
            &Document {
                serial: "9999".to_string(),
                number: "999991".to_string(),
            },
        )
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    let _ = service
        .passport_confirm(
            &"qwgzenvrwy@outlook.com".to_string(),
            &Document {
                serial: "9999".to_string(),
                number: "999999".to_string(),
            },
        )
        .await;
}
