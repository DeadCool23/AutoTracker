use lazy_static::lazy_static;
use std::collections::HashMap;

use business_logic::services::auth_service::AuthService;
use business_logic::services::route_service::RouteService;
use business_logic::services::search_service::SearchService;
use business_logic::services::snap_send_service::SnapSendService;
use business_logic::services_traits::{Authorizer, RouteGetter, Searcher, SnapSender};
use data_access::repositories::mocked::*;

pub use business_logic::error;

pub enum BLServices {
    RouteGetService(Box<dyn RouteGetter>),
    AuthService(Box<dyn Authorizer>),
    SnapSendService(Box<dyn SnapSender>),
    SearchService(Box<dyn Searcher>),
}

lazy_static! {
    pub static ref BUSINESS_PROCCESS: HashMap<String, BLServices> = HashMap::from([
        (
            "route_getter".to_string(),
            BLServices::RouteGetService(Box::new(RouteService::from(
                Box::new(MockSnapRepo),
                Box::new(MockTrackInfoRepo),
            )))
        ),
        (
            "auther".to_string(),
            BLServices::AuthService(Box::new(AuthService::from(Box::new(MockUserRepo))))
        ),
        (
            "snap_sender".to_string(),
            BLServices::SnapSendService(Box::new(SnapSendService::from(Box::new(MockSnapRepo))))
        ),
        (
            "searcher".to_string(),
            BLServices::SearchService(Box::new(SearchService::from(
                Box::new(MockCarRepo),
                Box::new(MockTrackInfoRepo)
            )))
        )
    ]);
}
