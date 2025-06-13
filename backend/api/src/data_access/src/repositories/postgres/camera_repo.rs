use crate::error::DataAccessError;
use crate::repositories_traits::CameraRepository;
use async_trait::async_trait;
use models::{Camera, Location};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

pub struct PgCameraRepo {
    pool: PgPool,
}

impl PgCameraRepo {
    pub async fn from(pg_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Connecting to PostgreSQL database for Camera repository");
        let pool = PgPoolOptions::new()
            .max_connections(super::MAX_CONNECTIONS)
            .acquire_timeout(super::CONNECTION_WATING_TIME)
            .connect(pg_url)
            .await
            .map_err(|e| {
                log::error!("Failed to connect to PostgreSQL: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?;

        log::info!("Successfully connected to PostgreSQL");
        Ok(PgCameraRepo { pool })
    }
}

impl PgCameraRepo {
    fn from_row_to_camera(row: PgRow) -> Camera {
        Camera {
            id: row.get::<i32, _>("id") as usize,
            is_radar: row.get("is_radar"),
            location: Location {
                longitude: row.get("longitude"),
                latitude: row.get("latitude"),
            },
        }
    }
}

#[async_trait]
impl CameraRepository for PgCameraRepo {
    async fn get_camera_count(&self) -> Result<usize, DataAccessError> {
        log::info!("Getting total camera count");
        let query = "SELECT COUNT(*) FROM Camera";
        log::debug!("Executing query: {}", query);

        let count: i64 = sqlx::query(query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                log::error!("Failed to get camera count: {}", e);
                DataAccessError::PsqlDataBaseError(e)
            })?
            .get(0);

        log::info!("Total cameras found: {}", count);
        Ok(count as usize)
    }

    async fn get_camera_by_id(&self, id: usize) -> Result<Camera, DataAccessError> {
        log::info!("Getting camera by ID: {}", id);
        let query = "SELECT id, is_radar, longitude, latitude FROM Camera WHERE id = $1";
        log::debug!("Executing query: {} with ID: {}", query, id);

        let row = sqlx::query(query)
            .bind(id as i32)
            .fetch_one(&self.pool)
            .await;

        match row {
            Ok(row) => {
                let camera = Self::from_row_to_camera(row);
                log::debug!("Getted camera: {:?}", camera);
                Ok(camera)
            }
            Err(sqlx::Error::RowNotFound) => {
                log::warn!("Camera with ID {} not found", id);
                Err(DataAccessError::NotFoundError("camera".to_string()))
            }
            Err(e) => {
                log::error!("Failed to get camera with ID {}: {}", id, e);
                Err(DataAccessError::PsqlDataBaseError(e))
            }
        }
    }

    async fn get_camera_by_location(&self, location: &Location) -> Result<Camera, DataAccessError> {
        log::info!("Getting camera with location: {:#?}", location);
        let query = "
            SELECT id, is_radar, longitude, latitude 
            FROM Camera 
            WHERE longitude = $1 AND latitude = $2";
        log::debug!("Executing query: {} with location: {:?}", query, location);

        let row = sqlx::query(query)
            .bind(location.longitude)
            .bind(location.latitude)
            .fetch_one(&self.pool)
            .await;

        match row {
            Ok(row) => {
                let camera = Self::from_row_to_camera(row);
                log::debug!("Getted camera: {:?}", camera);
                Ok(camera)
            }
            Err(sqlx::Error::RowNotFound) => {
                log::warn!("Camera with location {:?} not found", location);
                Err(DataAccessError::NotFoundError("camera".to_string()))
            }
            Err(e) => {
                log::error!("Failed to get camera with location {:?}: {}", location, e);
                Err(DataAccessError::PsqlDataBaseError(e))
            }
        }
    }

    async fn get_avg_speed_for_car_at_camera(
        &self,
        gos_num: &str,
        cam_id: usize,
    ) -> Result<f64, DataAccessError> {
        log::info!(
            "Getting avg speed for car with gos_num {} on camera with ID: {}",
            gos_num,
            cam_id
        );

        let query = "
            SELECT get_avg_speed_for_car_at_camera($1, $2) AS avg_speed
        ";
        log::debug!(
            "Executing query: {} with car gos_num: {} and camera ID: {}",
            query,
            gos_num,
            cam_id
        );

        let row = sqlx::query(query)
            .bind(gos_num)
            .bind(cam_id as i32)
            .fetch_one(&self.pool)
            .await?;

        let avg_speed: Option<f64> = row.get("avg_speed");
        log::info!("Getted avg speed {:?}", avg_speed);
        Ok(avg_speed.unwrap_or(0.))
    }
}
