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

use data_access::repositories_traits::*;

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
            "route_getter" => Self::build_route_get_service().await,
            "auther" => Self::build_auth_service().await,
            "searcher" => Self::build_search_service().await,
            "snap_sender" => Self::build_snap_send_service().await,
            "camera_data_getter" => Self::build_camera_data_get_service().await,
            _ => None,
        }
    }

    async fn get_snap_repo() -> Option<Box<dyn SnapRepository>> {
        match DataContainer::get("snap_repo").await {
            Some(Repositories::SnapRepo(repo)) => {
                log::info!("Successfully got SnapRepository");
                Some(repo)
            }
            Some(_) => {
                log::error!("Got incorrect repository for SnapRepository");
                None
            }
            None => {
                log::error!("Can't get SnapRepository");
                None
            }
        }
    }

    async fn get_user_repo() -> Option<Box<dyn UserRepository>> {
        match DataContainer::get("user_repo").await {
            Some(Repositories::UserRepo(repo)) => {
                log::info!("Successfully got UserRepository");
                Some(repo)
            }
            Some(_) => {
                log::error!("Getted incorrect repository for UserRepository");
                None
            }
            None => {
                log::error!("Can't get UserRepository");
                None
            }
        }
    }

    async fn get_car_repo() -> Option<Box<dyn CarRepository>> {
        match DataContainer::get("car_repo").await {
            Some(Repositories::CarRepo(repo)) => {
                log::info!("Successfully got CarRepository");
                Some(repo)
            }
            Some(_) => {
                log::error!("Getted incorrect repository for CarRepository");
                None
            }
            None => {
                log::error!("Can't get CarRepository");
                None
            }
        }
    }

    async fn get_track_info_repo() -> Option<Box<dyn TrackInfoRepository>> {
        match DataContainer::get("track_info_repo").await {
            Some(Repositories::TrackInfoRepo(repo)) => {
                log::info!("Successfully got TrackInfoRepository");
                Some(repo)
            }
            Some(_) => {
                log::error!("Getted incorrect repository for TrackInfoRepository");
                None
            }
            None => {
                log::error!("Can't get TrackInfoRepository");
                None
            }
        }
    }

    async fn get_camera_repo() -> Option<Box<dyn CameraRepository>> {
        match DataContainer::get("camera_repo").await {
            Some(Repositories::CameraRepo(repo)) => {
                log::info!("Successfully got CameraRepository");
                Some(repo)
            }
            Some(_) => {
                log::error!("Getted incorrect repository for CameraRepository");
                None
            }
            None => {
                log::error!("Can't get CameraRepository");
                None
            }
        }
    }

    async fn build_route_get_service() -> Option<CoreServices> {
        let snap_repo = Self::get_snap_repo().await?;
        log::info!("Successfull getted SnapRepository");
        let track_info_repo = Self::get_track_info_repo().await?;
        log::info!("Successfull getted TrackInfoRepository");
        let user_repo = Self::get_user_repo().await?;
        log::info!("Successfull getted UserRepository");

        log::info!("Sending RouteGetter");
        Some(CoreServices::RouteGetService(Box::new(RouteService::from(
            user_repo,
            snap_repo,
            track_info_repo,
        ))))
    }

    async fn build_auth_service() -> Option<CoreServices> {
        let user_repo = Self::get_user_repo().await?;
        log::info!("Successfull getted UserRepository");

        log::info!("Sending Auther");
        Some(CoreServices::AuthService(Box::new(AuthService::from(
            user_repo,
        ))))
    }

    async fn build_search_service() -> Option<CoreServices> {
        let car_repo = Self::get_car_repo().await?;
        log::info!("Successfull getted CarRepository");
        let track_info_repo = Self::get_track_info_repo().await?;
        log::info!("Successfull getted TrackInfoRepository");

        log::info!("Sending Searcher");
        Some(CoreServices::SearchService(Box::new(SearchService::from(
            car_repo,
            track_info_repo,
        ))))
    }

    async fn build_snap_send_service() -> Option<CoreServices> {
        let snap_repo = Self::get_snap_repo().await?;
        log::info!("Successfull getted SnapRepository");

        log::info!("Sending SnapSender");
        Some(CoreServices::SnapSendService(Box::new(
            SnapSendService::from(snap_repo),
        )))
    }

    async fn build_camera_data_get_service() -> Option<CoreServices> {
        let camera_repo = Self::get_camera_repo().await?;
        log::info!("Successfull getted CameraRepository");

        log::info!("Sending CameraDataGetter");
        Some(CoreServices::CameraDataGetService(Box::new(
            CameraDataGetService::from(camera_repo),
        )))
    }
}
