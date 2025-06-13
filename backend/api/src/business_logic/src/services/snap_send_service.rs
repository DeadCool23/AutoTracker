use super::validator::Validator;
use crate::error::ServiceError;
use async_trait::async_trait;
use data_access::repositories_traits::SnapRepository;
use models::{Camera, Snap};

use crate::services_traits;

pub struct SnapSendService {
    snap_repo: Box<dyn SnapRepository>,
}

impl SnapSendService {
    pub fn from(snap_repo: Box<dyn SnapRepository>) -> Self {
        SnapSendService { snap_repo }
    }
}

unsafe impl Send for SnapSendService {}
unsafe impl Sync for SnapSendService {}

#[async_trait]
impl services_traits::SnapSender for SnapSendService {
    async fn insert_snap(
        &self,
        camera: &Camera,
        speed: Option<u16>,
        time: &String,
        date: &String,
        gos_num: &String,
    ) -> Result<(), ServiceError> {
        log::info!(
            "Starting snap insertion for vehicle {} at {} {}",
            gos_num,
            date,
            time
        );

        if !Validator::is_valid_gos_num(gos_num) {
            log::warn!("Invalid vehicle number format: {}", gos_num);
            return Err(ServiceError::InvalidDataError("gos number".to_string()));
        }

        if !Validator::is_valid_date(date) {
            log::warn!("Invalid date format: {}", date);
            return Err(ServiceError::InvalidDataError("date".to_string()));
        }

        if !Validator::is_valid_time(time) {
            log::warn!("Invalid time format: {}", time);
            return Err(ServiceError::InvalidDataError("time".to_string()));
        }

        log::debug!("Creating snap object for camera ID: {}", camera.id);
        let snap = Snap {
            camera: *camera,
            speed: speed.clone(),
            time: time.clone(),
            date: date.clone(),
            gos_num: gos_num.clone(),
        };

        log::debug!("Inserting snap into repository");
        match self.snap_repo.insert_snap(&snap).await {
            Ok(_) => {
                log::info!(
                    "Successfully inserted snap for vehicle {} at {} {}",
                    gos_num,
                    date,
                    time
                );
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to insert snap: {}", e);
                Err(ServiceError::DataAccessError(e))
            }
        }
    }
}
