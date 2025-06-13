use super::create_clickhouse_client;
use crate::error::DataAccessError;
use crate::repositories_traits::CameraRepository;
use async_trait::async_trait;
use clickhouse::{Client, Row};
use models::{Camera, Location};
use serde::Deserialize;

pub struct ClickHouseCameraRepo {
    client: Client,
}

impl ClickHouseCameraRepo {
    pub async fn from(clickhouse_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Connecting to ClickHouse database for Camera repository");

        let client = create_clickhouse_client(clickhouse_url);

        log::info!("Successfully connected to ClickHouse");
        Ok(Self { client })
    }
}

#[derive(Debug, Deserialize, Row)]
struct CountRow {
    cnt: u64,
}

#[derive(Debug, Deserialize, Row)]
struct CameraRow {
    id: u32,
    is_radar: bool,
    longitude: f64,
    latitude: f64,
}

#[derive(Debug, Deserialize, Row)]
struct AvgSpeedRow {
    avg_speed: Option<f64>,
}

impl ClickHouseCameraRepo {
    fn from_row_to_camera(camera: CameraRow) -> Camera {
        Camera {
            id: camera.id as usize,
            is_radar: camera.is_radar,
            location: Location {
                longitude: camera.longitude,
                latitude: camera.latitude,
            },
        }
    }
}

#[async_trait]
impl CameraRepository for ClickHouseCameraRepo {
    async fn get_camera_count(&self) -> Result<usize, DataAccessError> {
        log::info!("Getting total camera count");

        let query = "SELECT count() as cnt FROM Camera";

        let row = self
            .client
            .query(query)
            .fetch_one::<CountRow>()
            .await
            .map_err(|e| {
                log::error!("Failed to get camera count: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Total cameras found: {}", row.cnt);
        Ok(row.cnt as usize)
    }

    async fn get_camera_by_id(&self, id: usize) -> Result<Camera, DataAccessError> {
        log::info!("Getting camera by ID: {}", id);

        let query = "SELECT id, is_radar, longitude, latitude FROM Camera WHERE id = ? LIMIT 1";

        let camera = self
            .client
            .query(query)
            .bind(id as u32)
            .fetch_one::<CameraRow>()
            .await
            .map_err(|e| {
                log::error!("Failed to get camera by id: {}", e);
                DataAccessError::NotFoundError("camera".into())
            })?;

        Ok(Self::from_row_to_camera(camera))
    }

    async fn get_camera_by_location(&self, location: &Location) -> Result<Camera, DataAccessError> {
        log::info!("Getting camera by location: {:?}", location);

        let query = "
            SELECT id, is_radar, longitude, latitude
            FROM Camera
            WHERE longitude = ? AND latitude = ?
            LIMIT 1
        ";

        let camera = self
            .client
            .query(query)
            .bind(location.longitude)
            .bind(location.latitude)
            .fetch_one::<CameraRow>()
            .await
            .map_err(|e| {
                log::error!("Failed to get camera by location: {}", e);
                DataAccessError::NotFoundError("camera".into())
            })?;

        Ok(Self::from_row_to_camera(camera))
    }

    async fn get_avg_speed_for_car_at_camera(
        &self,
        gos_num: &str,
        cam_id: usize,
    ) -> Result<f64, DataAccessError> {
        log::info!(
            "Getting avg speed for car '{}' on camera ID {}",
            gos_num,
            cam_id
        );

        let query = "
            SELECT avg(speed) AS avg_speed
            FROM CarSnapshot
            WHERE gos_num = ? AND camera_id = ?
        ";

        let row = self
            .client
            .query(query)
            .bind(gos_num)
            .bind(cam_id as u32)
            .fetch_one::<AvgSpeedRow>()
            .await?;

        Ok(row.avg_speed.unwrap_or(0.))
    }
}
