use data_access::{
    repositories::{
        clickhouse::{ClickHouseTrackInfoRepo, CLICKHOUSE_URL},
        postgres::{PgTrackInfoRepo, PG_URL},
    },
    repositories_traits::TrackInfoRepository,
};
use models::Document;

#[tokio::test]
async fn test_get_track_infos_by_gos_num_mask() {
    let repo = PgTrackInfoRepo::from(&PG_URL).await.unwrap();

    let res = repo.get_track_info_by_car_gos_number_mask("*5****77").await;
    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_track_infos_by_passport() {
    let repo = PgTrackInfoRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_track_info_by_user_passport(&Document {
            serial: "3063".to_string(),
            number: "208923".to_string(),
        })
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_track_infos_by_fio() {
    let repo = PgTrackInfoRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_track_info_by_user_fio(None, Some("Дроздов"), None)
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_track_infos_by_date() {
    let repo = PgTrackInfoRepo::from(&PG_URL).await.unwrap();

    let res = repo.get_track_info_by_date("22.04.2025").await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_track_infos_by_filter1() {
    let repo = PgTrackInfoRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_tracks_info_by_filters(None, None, None, None, None, None)
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_track_infos_by_filter2() {
    let repo = PgTrackInfoRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_tracks_info_by_filters(None, None, None, None, None, Some("22.04.2025"))
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_track_infos_by_filter3() {
    let repo = PgTrackInfoRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_tracks_info_by_filters(Some("Герасим"), Some("Игнатьева"), None, None, None, Some("22.04.2025"))
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_get_track_infos_by_filter4() {
    let repo = PgTrackInfoRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .get_tracks_info_by_filters(Some("Герасим"), Some("Игнатьева"), None, None, Some("Е******"), Some("22.04.2025"))
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}


#[tokio::test]
async fn test_insert_track_info() {
    let repo = PgTrackInfoRepo::from(&PG_URL).await.unwrap();

    let res = repo
        .insert_track_info("О777ОО77", "example@example.com", "01.01.2020")
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_insert_track_info() {
    let repo = ClickHouseTrackInfoRepo::from(&CLICKHOUSE_URL)
        .await
        .unwrap();

    let res = repo
        .insert_track_info("Е880ХС65", "b35hvde3@mail.com", "25.07.2024")
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_track_infos_by_date() {
    let repo = ClickHouseTrackInfoRepo::from(&CLICKHOUSE_URL)
        .await
        .unwrap();

    let res = repo.get_track_info_by_date("19.04.2025").await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_track_infos_by_fio() {
    let repo = ClickHouseTrackInfoRepo::from(&CLICKHOUSE_URL)
        .await
        .unwrap();

    let res = repo
        .get_track_info_by_user_fio(None, Some("Никонова"), None)
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_track_infos_by_passport() {
    let repo = ClickHouseTrackInfoRepo::from(&CLICKHOUSE_URL)
        .await
        .unwrap();

    let res = repo
        .get_track_info_by_user_passport(&Document {
            serial: "8991".to_string(),
            number: "872982".to_string(),
        })
        .await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_track_infos_by_gos_num_mask() {
    let repo = ClickHouseTrackInfoRepo::from(&CLICKHOUSE_URL)
        .await
        .unwrap();

    let res = repo.get_track_info_by_car_gos_number_mask("*******").await;
    println!("{:#?}", res);
    assert!(res.is_ok())
}
