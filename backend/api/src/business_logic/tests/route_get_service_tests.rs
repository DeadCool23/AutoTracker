use business_logic::services::route_service::RouteService;
use business_logic::services_traits::RouteGetter;
use data_access::repositories::mocked::{MockSnapRepo, MockTrackInfoRepo, MockUserRepo};

#[tokio::test]
async fn test_handle_route_success() {
    let service = RouteService::from(
        Box::new(MockUserRepo),
        Box::new(MockSnapRepo),
        Box::new(MockTrackInfoRepo),
    );

    let res = service
        .get_car_route(
            &"А777МР77".to_string(),
            &"exist@exist.com".to_string(),
            &"01.01.2025".to_string(),
        )
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_route_invalid_date() {
    let service = RouteService::from(
        Box::new(MockUserRepo),
        Box::new(MockSnapRepo),
        Box::new(MockTrackInfoRepo),
    );

    let res = service
        .get_car_route(
            &"А777МР77".to_string(),
            &"example@example.com".to_string(),
            &"0101.2025".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: date");
}

#[tokio::test]
async fn test_handle_route_invalid_gos_num() {
    let service = RouteService::from(
        Box::new(MockUserRepo),
        Box::new(MockSnapRepo),
        Box::new(MockTrackInfoRepo),
    );

    let res = service
        .get_car_route(
            &"А777Р77".to_string(),
            &"example@example.com".to_string(),
            &"01.01.2025".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: gos number");
}

#[tokio::test]
async fn test_handle_route_invalid_email() {
    let service = RouteService::from(
        Box::new(MockUserRepo),
        Box::new(MockSnapRepo),
        Box::new(MockTrackInfoRepo),
    );

    let res = service
        .get_car_route(
            &"А777МР77".to_string(),
            &"aexample.com".to_string(),
            &"01.01.2025".to_string(),
        )
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: email");
}
