use crate::error::ServiceError;
use crate::services_traits::CameraDataGetter;
use async_trait::async_trait;
use data_access::{error::DataAccessError, repositories_traits::CameraRepository};
use models::{Camera, Location};

use super::validator::Validator;

pub struct CameraDataGetService {
    cam_repo: Box<dyn CameraRepository>,
}

impl CameraDataGetService {
    pub fn from(cam_repo: Box<dyn CameraRepository>) -> Self {
        CameraDataGetService { cam_repo }
    }
}

unsafe impl Send for CameraDataGetService {}
unsafe impl Sync for CameraDataGetService {}

#[async_trait]
impl CameraDataGetter for CameraDataGetService {
    async fn get_camera_by_id(&self, id: usize) -> Result<Camera, ServiceError> {
        log::info!("Attempting getting camera with ID: {}", id);

        let cam = match self.cam_repo.get_camera_by_id(id).await {
            Ok(cam) => cam,
            Err(DataAccessError::NotFoundError(e)) => {
                return Err(ServiceError::NotFoundError(e));
            }
            Err(e) => {
                return Err(ServiceError::DataAccessError(e));
            }
        };

        log::info!("Getted camera: {:#?}", cam);
        Ok(cam)
    }
    async fn get_camera_by_location(&self, location: &Location) -> Result<Camera, ServiceError> {
        log::info!("Attempting getting camera with location: {:#?}", location);

        let cam = match self.cam_repo.get_camera_by_location(location).await {
            Ok(cam) => cam,
            Err(DataAccessError::NotFoundError(e)) => {
                return Err(ServiceError::NotFoundError(e));
            }
            Err(e) => {
                return Err(ServiceError::DataAccessError(e));
            }
        };

        log::info!("Getted camera: {:#?}", cam);
        Ok(cam)
    }
    async fn get_avg_speed_of_car_on_camera_by_gos_num(
        &self,
        gos_num: &String,
        location: &Location,
    ) -> Result<f64, ServiceError> {
        if !Validator::is_valid_gos_num(gos_num) {
            log::warn!("Invalid vehicle number format: {}", gos_num);
            return Err(ServiceError::InvalidDataError("gos number".to_string()));
        }

        log::info!("Getting camera data by location: {:?}", location);
        let cam = match self.cam_repo.get_camera_by_location(location).await {
            Ok(cam) => cam,
            Err(DataAccessError::NotFoundError(e)) => {
                return Err(ServiceError::NotFoundError(e));
            }
            Err(e) => {
                return Err(ServiceError::DataAccessError(e));
            }
        };

        if !cam.is_radar {
            log::warn!("Camera on laction: {:?} don't have radar", location);
            return Err(ServiceError::NotFoundError("average speed".to_string()));
        }

        log::info!(
            "Getting average speed for car with gos_num: {} on camera with id: {}",
            gos_num,
            cam.id
        );
        let avg_speed = self
            .cam_repo
            .get_avg_speed_for_car_at_camera(gos_num, cam.id)
            .await?;

        log::info!("Getted average speed: {}", avg_speed);
        Ok(avg_speed)
    }
}
