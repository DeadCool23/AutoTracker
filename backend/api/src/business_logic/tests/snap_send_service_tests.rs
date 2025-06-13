use business_logic::services::snap_send_service::SnapSendService;
use business_logic::services_traits::SnapSender;
use data_access::repositories::mocked::MockSnapRepo;
use models::{Camera, Location};

#[tokio::test]
async fn test_handle_snap_send_success() {
    let service = SnapSendService::from(Box::new(MockSnapRepo));

    let res = service
        .insert_snap(
            &Camera {
                id: 1,
                is_radar: true,
                location: Location {
                    longitude: 53.9333,
                    latitude: 53.9333,
                },
            },
            Some(70),
            &"8:10".to_string(),
            &"01.01.2025".to_string(),
            &"А777МР77".to_string(),
        )
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_snap_send_invalid_gos_num() {
    let service = SnapSendService::from(Box::new(MockSnapRepo));

    let res = service
        .insert_snap(
            &Camera {
                id: 1,
                is_radar: true,
                location: Location {
                    longitude: 53.9333,
                    latitude: 53.9333,
                },
            },
            Some(70),
            &"8:10".to_string(),
            &"01.01.2025".to_string(),
            &"А777Р77".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: gos number");
}

#[tokio::test]
async fn test_handle_snap_send_invalid_date() {
    let service = SnapSendService::from(Box::new(MockSnapRepo));

    let res = service
        .insert_snap(
            &Camera {
                id: 1,
                is_radar: true,
                location: Location {
                    longitude: 53.9333,
                    latitude: 53.9333,
                },
            },
            Some(70),
            &"8:10".to_string(),
            &"0101.2025".to_string(),
            &"А777МР77".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: date");
}

#[tokio::test]
async fn test_handle_snap_send_invalid_time_format() {
    let service = SnapSendService::from(Box::new(MockSnapRepo));

    let res = service
        .insert_snap(
            &Camera {
                id: 1,
                is_radar: true,
                location: Location {
                    longitude: 53.9333,
                    latitude: 53.9333,
                },
            },
            Some(70),
            &"8-10".to_string(),
            &"01.01.2025".to_string(),
            &"А777МР77".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: time");
}

#[tokio::test]
async fn test_handle_snap_send_time_value() {
    let service = SnapSendService::from(Box::new(MockSnapRepo));

    let res = service
        .insert_snap(
            &Camera {
                id: 1,
                is_radar: true,
                location: Location {
                    longitude: 53.9333,
                    latitude: 53.9333,
                },
            },
            Some(70),
            &"25:10".to_string(),
            &"01.01.2025".to_string(),
            &"А777МР77".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: time");
}
