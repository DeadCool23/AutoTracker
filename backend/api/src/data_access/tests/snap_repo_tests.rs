use data_access::{
    repositories::{
        clickhouse::{ClickHouseSnapRepo, CLICKHOUSE_URL},
        postgres::{PgSnapRepo, PG_URL},
        redis::{RedisSnapRepo, REDIS_URL},
        tandem::TandemSnapRepo,
    },
    repositories_traits::SnapRepository,
};
use models::{Camera, Location, Snap};

#[tokio::test]
async fn test_pg_get_snaps_by_date() {
    let repo = PgSnapRepo::from(&PG_URL).await.unwrap();

    let res = repo.get_car_snaps_by_date("А889МН29", "11.05.2024").await;

    println!("{:?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_pg_insert_snap() {
    let repo = PgSnapRepo::from(&PG_URL).await.unwrap();
    let _snap = Snap {
        camera: Camera {
            id: 1,
            is_radar: true,
            location: Location {
                latitude: 12.22222,
                longitude: 12.22222,
            },
        },
        speed: Some(70),
        time: "12:12".to_string(),
        date: "10.10.2020".to_string(),
        gos_num: "А889МН29".to_string(),
    };

    let res = repo.insert_snap(&_snap).await;
    let _ = repo.delete_snap(&_snap).await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_redis_get_snaps_by_date() {
    let repo = RedisSnapRepo::from(&REDIS_URL).unwrap();

    let res = repo.get_car_snaps_by_date("А889МН29", "11.05.2024").await;

    println!("{:?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_redis_insert_snap() {
    let repo = RedisSnapRepo::from(&REDIS_URL).unwrap();
    let _snap = Snap {
        camera: Camera {
            id: 1,
            is_radar: true,
            location: Location {
                latitude: 12.22222,
                longitude: 12.22222,
            },
        },
        speed: Some(70),
        time: "12:12".to_string(),
        date: "10.10.2020".to_string(),
        gos_num: "А889МН29".to_string(),
    };

    let res = repo.insert_snap(&_snap).await;
    let _ = repo.delete_snap(&_snap).await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_tandem_get_snaps_by_date() {
    let repo = TandemSnapRepo::from(
        Box::new(PgSnapRepo::from(&PG_URL).await.unwrap()),
        Box::new(RedisSnapRepo::from(&REDIS_URL).unwrap()),
    );

    let res = repo.get_car_snaps_by_date("А889МН29", "11.05.2024").await;

    println!("{:?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_tandem_insert_snap() {
    let repo = TandemSnapRepo::from(
        Box::new(PgSnapRepo::from(&PG_URL).await.unwrap()),
        Box::new(RedisSnapRepo::from(&REDIS_URL).unwrap()),
    );

    let _snap = Snap {
        camera: Camera {
            id: 1,
            is_radar: true,
            location: Location {
                latitude: 12.22222,
                longitude: 12.22222,
            },
        },
        speed: Some(70),
        time: "12:12".to_string(),
        date: "10.10.2020".to_string(),
        gos_num: "А889МН29".to_string(),
    };

    let res = repo.insert_snap(&_snap).await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_get_snaps_by_date() {
    let repo = ClickHouseSnapRepo::from(&CLICKHOUSE_URL).await.unwrap();

    let res = repo.get_car_snaps_by_date("А889МН29", "11.05.2024").await;

    println!("{:?}", res);
    assert!(res.is_ok())
}

#[tokio::test]
async fn test_clickhouse_insert_snap() {
    let repo = ClickHouseSnapRepo::from(&CLICKHOUSE_URL).await.unwrap();
    let _snap = Snap {
        camera: Camera {
            id: 1,
            is_radar: true,
            location: Location {
                latitude: 12.22222,
                longitude: 12.22222,
            },
        },
        speed: Some(70),
        time: "12:12".to_string(),
        date: "10.10.2020".to_string(),
        gos_num: "А889МН29".to_string(),
    };

    let res = repo.insert_snap(&_snap).await;
    let _ = repo.delete_snap(&_snap).await;

    println!("{:#?}", res);
    assert!(res.is_ok())
}
