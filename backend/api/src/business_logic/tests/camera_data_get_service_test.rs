use business_logic::services::camera_data_get_service::CameraDataGetService;
use business_logic::services_traits::CameraDataGetter;
use data_access::repositories::mocked::MockCameraRepo;
use models::Location;

#[tokio::test]
async fn test_handle_search_camera_by_id_success() {
    let service = CameraDataGetService::from(Box::new(MockCameraRepo));

    let res = service.get_camera_by_id(1).await;

    println!("{:?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_search_camera_by_cords_success() {
    let service = CameraDataGetService::from(Box::new(MockCameraRepo));

    let res = service
        .get_camera_by_location(&Location {
            latitude: 55.573816,
            longitude: 37.566005,
        })
        .await;

    println!("{:?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_get_avg_speed_of_car_on_camera_by_gos_num_success() {
    let service = CameraDataGetService::from(Box::new(MockCameraRepo));

    let res = service
        .get_avg_speed_of_car_on_camera_by_gos_num(
            &"О987МС36".to_string(),
            &Location {
                latitude: 55.573816,
                longitude: 37.566005,
            },
        )
        .await;

    println!("{:?}", res);
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_get_avg_speed_of_car_on_camera_by_gos_num_incorrect_gos_num() {
    let service = CameraDataGetService::from(Box::new(MockCameraRepo));

    let res = service
        .get_avg_speed_of_car_on_camera_by_gos_num(
            &"О98МС36".to_string(),
            &Location {
                latitude: 55.573816,
                longitude: 37.566005,
            },
        )
        .await;

    println!("{:?}", res);
    assert!(res.is_err());
}
