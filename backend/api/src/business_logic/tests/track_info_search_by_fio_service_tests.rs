use business_logic::services::search_service::SearchService;
use business_logic::services_traits::TrackInfoSearcher;
use data_access::repositories::mocked::{MockCarRepo, MockTrackInfoRepo};

#[tokio::test]
async fn test_handle_search_track_info_by_fio_full_fio_success() {
    let service = SearchService::from(Box::new(MockCarRepo), Box::new(MockTrackInfoRepo));

    let res = service
        .search_track_info_by_owner_fio(
            Some("firstname".to_string()),
            Some("surname".to_string()),
            Some("lastname".to_string()),
        )
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_search_track_info_by_fio_without_name_success() {
    let service = SearchService::from(Box::new(MockCarRepo), Box::new(MockTrackInfoRepo));

    let res = service
        .search_track_info_by_owner_fio(
            None,
            Some("surname".to_string()),
            Some("lastname".to_string()),
        )
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_search_track_info_by_fio_without_surname_success() {
    let service = SearchService::from(Box::new(MockCarRepo), Box::new(MockTrackInfoRepo));

    let res = service
        .search_track_info_by_owner_fio(
            Some("firstname".to_string()),
            None,
            Some("lastname".to_string()),
        )
        .await;

    assert!(res.is_ok());
}

#[tokio::test]
async fn test_handle_search_track_info_by_fio_without_lastname_success() {
    let service = SearchService::from(Box::new(MockCarRepo), Box::new(MockTrackInfoRepo));

    let res = service
        .search_track_info_by_owner_fio(
            Some("firstname".to_string()),
            Some("surname".to_string()),
            None,
        )
        .await;

    assert!(res.is_ok());
}
