use business_logic::services::search_service::SearchService;
use business_logic::services_traits::TrackInfoSearcher;
use data_access::repositories::mocked::{MockCarRepo, MockTrackInfoRepo};

#[tokio::test]
async fn test_handle_search_track_info_by_date_success() {
    let service = SearchService::from(Box::new(MockCarRepo), Box::new(MockTrackInfoRepo));

    let res = service
        .search_track_info_by_date(&"01.01.2025".to_string())
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_search_track_info_by_date_invalid_date() {
    let service = SearchService::from(Box::new(MockCarRepo), Box::new(MockTrackInfoRepo));

    let res = service
        .search_track_info_by_date(&"0101.2025".to_string())
        .await;

    assert!(res.is_err());
    assert_eq!(res.err().unwrap().to_string(), "Invalid data: date");
}
