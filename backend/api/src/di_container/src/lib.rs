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
pub use repo_connect::{DARepos, DATA_ACCESSES};

pub enum BLServices {
    RouteGetService(Box<dyn RouteGetter>),
    AuthService(Box<dyn Authorizer>),
    SnapSendService(Box<dyn SnapSender>),
    SearchService(Box<dyn Searcher>),
    CameraDataGetService(Box<dyn CameraDataGetter>),
}

#[allow(non_camel_case_types)]
pub struct BUSINESS_SERVICES;

impl BUSINESS_SERVICES {
    pub async fn get(name: &str) -> Option<BLServices> {
        match name {
            "route_getter" => {
                let snap_repo = match DATA_ACCESSES::get("snap_repo").await {
                    Some(DARepos::SnapRepo(repo)) => repo,
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

                let track_info_repo = match DATA_ACCESSES::get("track_info_repo").await {
                    Some(DARepos::TrackInfoRepo(repo)) => repo,
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

                let user_repo = match DATA_ACCESSES::get("user_repo").await {
                    Some(DARepos::UserRepo(repo)) => repo,
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
                Some(BLServices::RouteGetService(Box::new(RouteService::from(
                    user_repo,
                    snap_repo,
                    track_info_repo,
                ))))
            }
            "auther" => {
                let user_repo = match DATA_ACCESSES::get("user_repo").await {
                    Some(DARepos::UserRepo(repo)) => repo,
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
                Some(BLServices::AuthService(Box::new(AuthService::from(
                    user_repo,
                ))))
            }
            "searcher" => {
                let car_repo = match DATA_ACCESSES::get("car_repo").await {
                    Some(DARepos::CarRepo(repo)) => repo,
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

                let track_info_repo = match DATA_ACCESSES::get("track_info_repo").await {
                    Some(DARepos::TrackInfoRepo(repo)) => repo,
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
                Some(BLServices::SearchService(Box::new(SearchService::from(
                    car_repo,
                    track_info_repo,
                ))))
            }
            "snap_sender" => {
                let snap_repo = match DATA_ACCESSES::get("snap_repo").await {
                    Some(DARepos::SnapRepo(repo)) => repo,
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
                Some(BLServices::SnapSendService(Box::new(
                    SnapSendService::from(snap_repo),
                )))
            }
            "camera_data_getter" => {
                let camera_repo = match DATA_ACCESSES::get("camera_repo").await {
                    Some(DARepos::CameraRepo(repo)) => repo,
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
                Some(BLServices::CameraDataGetService(Box::new(
                    CameraDataGetService::from(camera_repo),
                )))
            }
            _ => None,
        }
    }
}
