use business_logic::services::auth_service::AuthService;
use business_logic::services::camera_data_get_service::CameraDataGetService;
use business_logic::services::route_service::RouteService;
use business_logic::services::search_service::SearchService;
use business_logic::services::snap_send_service::SnapSendService;
use business_logic::services_traits::{
    Authorizer, CameraDataGetter, RouteGetter, Searcher, SnapSender,
};

pub use business_logic::error;

mod repo_connect;
pub use repo_connect::{DataContainer, Repositories};

pub enum CoreServices {
    RouteGetService(Box<dyn RouteGetter>),
    AuthService(Box<dyn Authorizer>),
    SnapSendService(Box<dyn SnapSender>),
    SearchService(Box<dyn Searcher>),
    CameraDataGetService(Box<dyn CameraDataGetter>),
}

pub struct ServicesContainer;

impl ServicesContainer {
    pub async fn get(name: &str) -> Option<CoreServices> {
        match name {
            "route_getter" => {
                let snap_repo = match DataContainer::get("snap_repo").await {
                    Some(Repositories::SnapRepo(repo)) => repo,
                    Some(_) => {
                        log::error!("Getted incorrect repository");
                        panic!("Getted incorrect repository");
                    }
                    None => {
                        log::error!("Can't get SnapRepository");
                        return None;
                    }
                };
                log::info!("Successfull getted SnapRepository");

                let track_info_repo = match DataContainer::get("track_info_repo").await {
                    Some(Repositories::TrackInfoRepo(repo)) => repo,
                    Some(_) => {
                        log::error!("Getted incorrect repository");
                        panic!("Getted incorrect repository");
                    }
                    None => {
                        log::error!("Can't get TrackInfoRepository");
                        return None;
                    }
                };
                log::info!("Successfull getted TrackInfoRepository");

                let user_repo = match DataContainer::get("user_repo").await {
                    Some(Repositories::UserRepo(repo)) => repo,
                    Some(_) => {
                        log::error!("Getted incorrect repository");
                        panic!("Getted incorrect repository");
                    }
                    None => {
                        log::error!("Can't get UserRepository");
                        return None;
                    }
                };
                log::info!("Successfull getted UserRepository");

                log::info!("Sending RouteGetter");
                Some(CoreServices::RouteGetService(Box::new(RouteService::from(
                    user_repo,
                    snap_repo,
                    track_info_repo,
                ))))
            }
            "auther" => {
                let user_repo = match DataContainer::get("user_repo").await {
                    Some(Repositories::UserRepo(repo)) => repo,
                    Some(_) => {
                        log::error!("Getted incorrect repository");
                        panic!("Getted incorrect repository");
                    }
                    None => {
                        log::error!("Can't get UserRepository");
                        return None;
                    }
                };
                log::info!("Successfull getted UserRepository");

                log::info!("Sending Auther");
                Some(CoreServices::AuthService(Box::new(AuthService::from(
                    user_repo,
                ))))
            }
            "searcher" => {
                let car_repo = match DataContainer::get("car_repo").await {
                    Some(Repositories::CarRepo(repo)) => repo,
                    Some(_) => {
                        log::error!("Getted incorrect repository");
                        panic!("Getted incorrect repository");
                    }
                    None => {
                        log::error!("Can't get CarRepository");
                        return None;
                    }
                };
                log::info!("Successfull getted CarRepository");

                let track_info_repo = match DataContainer::get("track_info_repo").await {
                    Some(Repositories::TrackInfoRepo(repo)) => repo,
                    Some(_) => {
                        log::error!("Getted incorrect repository");
                        panic!("Getted incorrect repository");
                    }
                    None => {
                        log::error!("Can't get TrackInfoRepository");
                        return None;
                    }
                };
                log::info!("Successfull getted TrackInfoRepository");

                log::info!("Sending Searcher");
                Some(CoreServices::SearchService(Box::new(SearchService::from(
                    car_repo,
                    track_info_repo,
                ))))
            }
            "snap_sender" => {
                let snap_repo = match DataContainer::get("snap_repo").await {
                    Some(Repositories::SnapRepo(repo)) => repo,
                    Some(_) => {
                        log::error!("Getted incorrect repository");
                        panic!("Getted incorrect repository");
                    }
                    None => {
                        log::error!("Can't get SnapRepository");
                        return None;
                    }
                };
                log::info!("Successfull getted SnapRepository");

                log::info!("Sending SnapSender");
                Some(CoreServices::SnapSendService(Box::new(
                    SnapSendService::from(snap_repo),
                )))
            }
            "camera_data_getter" => {
                let camera_repo = match DataContainer::get("camera_repo").await {
                    Some(Repositories::CameraRepo(repo)) => repo,
                    Some(_) => {
                        log::error!("Getted incorrect repository");
                        panic!("Getted incorrect repository");
                    }
                    None => {
                        log::error!("Can't get CameraRepository");
                        return None;
                    }
                };
                log::info!("Successfull getted CameraRepository");

                log::info!("Sending CameraDataGetter");
                Some(CoreServices::CameraDataGetService(Box::new(
                    CameraDataGetService::from(camera_repo),
                )))
            }
            _ => None,
        }
    }
}
