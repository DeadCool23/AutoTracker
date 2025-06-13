use data_access::{
    repositories::clickhouse::{ClickHouseCameraRepo, CLICKHOUSE_URL},
    repositories::postgres::{PgCameraRepo, PG_URL},
    repositories_traits::CameraRepository,
};
use models::Location;

#[tokio::test]
async fn test_get_cameras_cnt() {
    let repo = PgCameraRepo::from(&PG_URL).await.unwrap();

    let cnt = repo.get_camera_count().await;
    println!("{:?}", cnt);
    assert!(cnt.is_ok())
}

#[tokio::test]
async fn test_get_camera_by_id() {
    let repo = PgCameraRepo::from(&PG_URL).await.unwrap();

    let cnt = repo.get_camera_by_id(1).await;
    println!("{:?}", cnt);
    assert!(cnt.is_ok())
}

#[tokio::test]
async fn test_get_camera_by_cords() {
    let repo = PgCameraRepo::from(&PG_URL).await.unwrap();

    let cnt = repo
        .get_camera_by_location(&Location {
            latitude: 55.573816,
            longitude: 37.566005,
        })
        .await;
    println!("{:?}", cnt);
    assert!(cnt.is_ok())
}

#[tokio::test]
async fn test_get_avg_speed_on_camera() {
    let repo = PgCameraRepo::from(&PG_URL).await.unwrap();

    let avg_speed = repo.get_avg_speed_for_car_at_camera("О987МС36", 1).await;
    println!("{:?}", avg_speed);
    assert!(avg_speed.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_cameras_cnt() {
    let repo = ClickHouseCameraRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let cnt = repo.get_camera_count().await;
    println!("{:?}", cnt);
    assert!(cnt.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_camera_by_id() {
    let repo = ClickHouseCameraRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let cnt = repo.get_camera_by_id(1).await;
    println!("{:?}", cnt);
    assert!(cnt.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_camera_by_cords() {
    let repo = ClickHouseCameraRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let cnt = repo
        .get_camera_by_location(&Location {
            latitude: 55.573816,
            longitude: 37.566005,
        })
        .await;
    println!("{:?}", cnt);
    assert!(cnt.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_avg_speed_on_camera() {
    let repo = ClickHouseCameraRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let avg_speed = repo.get_avg_speed_for_car_at_camera("А202ХО49", 39).await;
    println!("{:?}", avg_speed);
    assert!(avg_speed.is_ok())
}
