use business_logic::services::search_service::SearchService;
use business_logic::services_traits::CarSearcher;
use data_access::repositories::mocked::{MockCarRepo, MockTrackInfoRepo};
use models::Document;

#[tokio::test]
async fn test_handle_search_car_by_passport_success() {
    let service = SearchService::from(Box::new(MockCarRepo), Box::new(MockTrackInfoRepo));

    let res = service
        .search_cars_by_owner_passport(&Document {
            serial: "1111".to_string(),
            number: "111111".to_string(),
        })
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_search_car_by_passport_invalid_passport_serial() {
    let service = SearchService::from(Box::new(MockCarRepo), Box::new(MockTrackInfoRepo));

    let res = service
        .search_cars_by_owner_passport(&Document {
            serial: "111".to_string(),
            number: "111111".to_string(),
        })
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: passport");
}

#[tokio::test]
async fn test_handle_search_car_by_passport_invalid_passport_number() {
    let service = SearchService::from(Box::new(MockCarRepo), Box::new(MockTrackInfoRepo));

    let res = service
        .search_cars_by_owner_passport(&Document {
            serial: "1111".to_string(),
            number: "11111".to_string(),
        })
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: passport");
}
