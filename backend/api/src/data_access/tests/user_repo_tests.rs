use data_access::{
    repositories::clickhouse::{ClickHouseUserRepo, CLICKHOUSE_URL},
    repositories::postgres::{PgUserRepo, PG_URL},
    repositories_traits::UserRepository,
};
use models::{Document, Role, User};

#[tokio::test]
async fn test_get_user_by_auth_info() {
    let repo = PgUserRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_user_by_auth_info("email123@example.com", "12345678")
        .await;
    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_user_by_email() {
    let repo = PgUserRepo::from(&PG_URL).await.unwrap();

    let res = repo.get_user_by_email("email123@example.com").await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_update_user_passport_fail() {
    let repo = PgUserRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .update_user_passport(
            &"email123@example.com".to_string(),
            &Document {
                serial: "1111".to_string(),
                number: "111111".to_string(),
            },
        )
        .await;

    assert!(res.is_err())
}

#[tokio::test]
async fn test_update_user_passport_success() {
    let email = "uewmleii@icloud.com".to_string();
    let passport = Document {
        serial: "1111".to_string(),
        number: "111111".to_string(),
    };
    let repo = PgUserRepo::from(&PG_URL).await.unwrap();

    let res = repo.update_user_passport(&email, &passport).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    let new_user = repo.get_user_by_email(&email).await.unwrap().unwrap();
    assert_eq!(new_user.clone().passport.unwrap().serial, passport.serial);
    assert_eq!(new_user.clone().passport.unwrap().number, passport.number);
}

#[tokio::test]
async fn test_add_user() {
    let new_user = User {
        name: "mock_name".to_string(),
        surname: "mock_surname".to_string(),
        lastname: None,
        email: "email123@example.com".to_string(),
        passport: None,
        role: Role::user,
        is_verified: false,
    };
    let pswd = "123456789";

    let repo = PgUserRepo::from(&PG_URL).await.unwrap();

    let res = repo.insert_user(&new_user, pswd).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    let res = repo.delete_user(&new_user).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_clickhouse_get_user_by_auth_info() {
    let repo = ClickHouseUserRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let res = repo
        .get_user_by_auth_info("uewmleii@icloud.com", "Krd!G0RW&")
        .await;
    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_user_by_email() {
    let repo = ClickHouseUserRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let res = repo.get_user_by_email("uewmleii@icloud.com").await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_update_user_passport_fail() {
    let repo = ClickHouseUserRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let res = repo
        .update_user_passport(
            &"email123@example.com".to_string(),
            &Document {
                serial: "1111".to_string(),
                number: "111111".to_string(),
            },
        )
        .await;

    println!("{:#?}", res);
    assert!(res.is_err())
}

#[tokio::test]
async fn test_clickhouse_update_user_passport_success() {
    let email = "uewmleii@icloud.com".to_string();
    let passport = Document {
        serial: "1111".to_string(),
        number: "111111".to_string(),
    };
    let repo = ClickHouseUserRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let res = repo.update_user_passport(&email, &passport).await;

    println!("{:#?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_clickhouse_add_user() {
    let new_user = User {
        name: "mock_name".to_string(),
        surname: "mock_surname".to_string(),
        lastname: None,
        email: "email123@example.com".to_string(),
        passport: None,
        role: Role::user,
        is_verified: false,
    };
    let pswd = "123456789";

    let repo = ClickHouseUserRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let res = repo.insert_user(&new_user, pswd).await;

    println!("{:#?}", res);
    assert!(res.is_ok());

    let res = repo.delete_user(&new_user).await;
    assert!(res.is_ok());
}
