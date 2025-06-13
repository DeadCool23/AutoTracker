use data_access::{
    repositories::{
        clickhouse::{ClickHouseCarRepo, CLICKHOUSE_URL},
        postgres::{PgCarRepo, PG_URL},
    },
    repositories_traits::CarRepository,
};
use models::Document;

#[tokio::test]
async fn test_get_cars_by_gos_num_mask() {
    let repo = PgCarRepo::from(&PG_URL).await.unwrap();

    let res = repo.get_car_by_gos_number_mask("*5****77").await;
    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_cars_by_passport() {
    let repo = PgCarRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_car_by_owner_passport(&Document {
            serial: "3063".to_string(),
            number: "208923".to_string(),
        })
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_cars_by_fio() {
    let repo = PgCarRepo::from(&PG_URL).await.unwrap();

    let res = repo.get_car_by_owner_fio(None, Some("Дроздов"), None).await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_cars_by_filter1() {
    let repo = PgCarRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_cars_by_filters(None, Some("Дроздов"), None, None, Some("*5****77"))
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_cars_by_filter2() {
    let repo = PgCarRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_cars_by_filters(None, Some("Дроздов"), None, None, None)
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_cars_by_filter3() {
    let repo = PgCarRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_cars_by_filters(Some("Парамон"), Some("Артемьева"), None, None, None)
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_cars_by_filter4() {
    let repo = PgCarRepo::from(&PG_URL).await.unwrap();

    let res = repo.get_cars_by_filters(None, None, None, None, None).await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_cars_by_gos_num_mask() {
    let repo = ClickHouseCarRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let res = repo.get_car_by_gos_number_mask("*5****77").await;
    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_cars_by_passport() {
    let repo = ClickHouseCarRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let res = repo
        .get_car_by_owner_passport(&Document {
            serial: "2725".to_string(),
            number: "379806".to_string(),
        })
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_cars_by_fio() {
    let repo = ClickHouseCarRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let res = repo.get_car_by_owner_fio(None, Some("Дроздов"), None).await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}
