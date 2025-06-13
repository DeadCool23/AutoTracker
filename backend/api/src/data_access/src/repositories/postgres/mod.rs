use lazy_static::lazy_static;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

mod camera_repo;
mod car_repo;
mod snap_repo;
mod track_info_repo;
mod user_repo;

pub use camera_repo::PgCameraRepo;
pub use car_repo::PgCarRepo;
pub use snap_repo::PgSnapRepo;
pub use track_info_repo::PgTrackInfoRepo;
pub use user_repo::PgUserRepo;

use crate::error::DataAccessError;

lazy_static! {
    pub static ref PG_URL: String = cfg::var("database.postgres_url");
}

const MAX_CONNECTIONS: u32 = 10000;
const CONNECTION_WATING_TIME: Duration = Duration::from_secs(1);

#[derive(Clone)]
pub struct PgConnectionManager {
    pg_url: String,
    max_retries: usize,
    retry_delay: Duration,
    connection: Arc<Mutex<Option<PgPool>>>,
}

impl PgConnectionManager {
    pub fn new(pg_url: String, max_retries: usize, retry_delay_secs: usize) -> Self {
        Self {
            pg_url,
            max_retries,
            retry_delay: Duration::from_secs(retry_delay_secs as u64),
            connection: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_connection(&self) -> Result<(), DataAccessError> {
        if let Some(_) = &*self.connection.lock().await {
            return Ok(());
        }

        for attempt in 1..=self.max_retries {
            match PgPoolOptions::new()
                .acquire_timeout(CONNECTION_WATING_TIME)
                .connect(&self.pg_url)
                .await
            {
                Ok(pool) => {
                    let mut guard = self.connection.lock().await;
                    *guard = Some(pool.clone());
                    return Ok(());
                }
                Err(e) => {
                    log::warn!(
                        "Попытка подключения {} из {} не удалась: {}",
                        attempt,
                        self.max_retries,
                        e
                    );
                    if attempt < self.max_retries {
                        tokio::time::sleep(self.retry_delay).await;
                    } else {
                        return Err(DataAccessError::PsqlDataBaseError(e));
                    }
                }
            }
        }

        return Err(DataAccessError::ReconnectionError);
    }

    pub async fn reconnect(&self) -> Result<(), DataAccessError> {
        *self.connection.lock().await = None;
        self.get_connection().await?;
        Ok(())
    }
}
