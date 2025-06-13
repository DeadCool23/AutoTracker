use crate::error::DataAccessError;
use crate::repositories_traits::{SnapRepoTransfer, SnapRepository, VolatileSnapRepo};

use async_trait::async_trait;
use chrono::NaiveDateTime;
use models::{Camera, Location, Snap};
use redis::{AsyncCommands, Client, RedisResult};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
struct RedisSnap {
    speed: Option<u16>,
    camera_id: usize,
    longitude: f64,
    latitude: f64,
    gos_num: String,
    datetime: String,
}

pub struct RedisSnapRepo {
    client: Client,
}

impl RedisSnapRepo {
    pub fn from(redis_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Creating RedisSnapRepo with URL: {}", redis_url);
        let client = Client::open(redis_url)?;
        Ok(RedisSnapRepo { client })
    }

    async fn get_connection(&self) -> RedisResult<redis::aio::MultiplexedConnection> {
        log::debug!("Acquiring Redis connection");
        self.client.get_multiplexed_async_connection().await
    }

    async fn get_keys_by_pattern(&self, pattern: &str) -> Result<Vec<String>, DataAccessError> {
        log::info!("Getting keys by pattern: {}", pattern);
        let mut conn = self.get_connection().await?;

        let mut keys = Vec::<String>::new();
        let mut iter = conn.scan_match(pattern).await?;

        while let Some(key) = iter.next_item().await {
            keys.push(key);
        }

        log::debug!("Found {} keys for pattern {}", keys.len(), pattern);
        Ok(keys)
    }

    fn form_snap_from_redis_snap(redis_snap: &RedisSnap) -> Result<Snap, DataAccessError> {
        log::debug!(
            "Converting RedisSnap to Snap for vehicle: {}",
            redis_snap.gos_num
        );
        let datetime = NaiveDateTime::parse_from_str(&redis_snap.datetime, "%d.%m.%Y %H:%M")
            .map_err(|e| {
                log::error!("Failed to parse datetime: {}", e);
                DataAccessError::InvalidInput(e.to_string())
            })?;

        Ok(Snap {
            camera: Camera {
                id: redis_snap.camera_id,
                is_radar: if redis_snap.speed.is_none() {
                    false
                } else {
                    true
                },
                location: Location {
                    longitude: redis_snap.longitude,
                    latitude: redis_snap.latitude,
                },
            },
            speed: redis_snap.speed,
            gos_num: redis_snap.gos_num.clone(),
            date: datetime.date().format("%d.%m.%Y").to_string(),
            time: datetime.time().format("%H:%M").to_string(),
        })
    }
}

#[async_trait]
impl SnapRepository for RedisSnapRepo {
    async fn insert_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        log::info!(
            "Inserting snap for vehicle {} at {} {}",
            snap.gos_num,
            snap.date,
            snap.time
        );
        let mut conn = self.get_connection().await?;

        let redis_snap = RedisSnap {
            speed: snap.speed,
            camera_id: snap.camera.id,
            longitude: snap.camera.location.longitude,
            latitude: snap.camera.location.latitude,
            gos_num: snap.gos_num.clone(),
            datetime: format!("{} {}", snap.date, snap.time),
        };

        let serialized = serde_json::to_string(&redis_snap).map_err(|e| {
            log::error!("Failed to serialize snap: {}", e);
            DataAccessError::InvalidInput(e.to_string())
        })?;

        let key = format!("snap:{}:{}", snap.gos_num, redis_snap.datetime);
        log::debug!("Storing snap with key: {}", key);

        let _: () = conn.set(key, serialized).await.map_err(|e| {
            log::error!("Redis set operation failed: {}", e);
            DataAccessError::RedisDataBaseError(e)
        })?;

        log::info!("Successfully inserted snap");
        Ok(())
    }

    async fn get_car_snaps_by_date(
        &self,
        gos_number: &str,
        date: &str,
    ) -> Result<Vec<Snap>, DataAccessError> {
        log::info!("Getting snaps for vehicle {} on date {}", gos_number, date);
        let mut conn = self.get_connection().await?;

        let pattern = format!("snap:{}:{}*", gos_number, date);
        log::debug!("Using key pattern: {}", pattern);

        let keys: Vec<String> = conn.keys(&pattern).await.map_err(|e| {
            log::error!("Redis keys operation failed: {}", e);
            DataAccessError::RedisDataBaseError(e)
        })?;

        log::debug!("Found {} matching keys", keys.len());
        let mut snaps = Vec::new();

        for key in keys {
            let serialized: String = conn.get(&key).await.map_err(|e| {
                log::error!("Redis get operation failed for key {}: {}", key, e);
                DataAccessError::RedisDataBaseError(e)
            })?;

            let redis_snap: RedisSnap = serde_json::from_str(&serialized).map_err(|e| {
                log::error!("Failed to deserialize snap from key {}: {}", key, e);
                DataAccessError::InvalidInput(e.to_string())
            })?;

            snaps.push(Self::form_snap_from_redis_snap(&redis_snap)?);
        }

        log::info!("Retrieved {} snaps", snaps.len());
        Ok(snaps)
    }
}

impl RedisSnapRepo {
    pub async fn delete_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        log::info!(
            "Deleting snap for vehicle {} at {} {}",
            snap.gos_num,
            snap.date,
            snap.time
        );
        let mut conn = self.get_connection().await?;

        let datetime = format!("{} {}", snap.date, snap.time);
        let key = format!("snap:{}:{}", snap.gos_num, datetime);
        log::debug!("Deleting key: {}", key);

        let _: usize = conn.del(&key).await.map_err(|e| {
            log::error!("Redis delete operation failed: {}", e);
            DataAccessError::RedisDataBaseError(e)
        })?;

        log::info!("Successfully deleted snap");
        Ok(())
    }
}

#[async_trait]
impl SnapRepoTransfer for RedisSnapRepo {
    async fn get_all_snaps(&self) -> Result<Vec<Snap>, DataAccessError> {
        log::info!("Getting all snaps from Redis");
        let mut snaps = Vec::new();
        let keys = self.get_keys_by_pattern("snap:*").await?;

        let mut conn = self.get_connection().await?;

        log::debug!("Processing {} snap keys", keys.len());
        for key in keys {
            let serialized: String = conn.get(&key).await.map_err(|e| {
                log::error!("Failed to get snap with key {}: {}", key, e);
                DataAccessError::RedisDataBaseError(e)
            })?;

            let redis_snap: RedisSnap = serde_json::from_str(&serialized).map_err(|e| {
                log::error!("Failed to deserialize snap with key {}: {}", key, e);
                DataAccessError::InvalidInput(e.to_string())
            })?;

            snaps.push(Self::form_snap_from_redis_snap(&redis_snap)?);
        }

        log::info!("Retrieved {} total snaps", snaps.len());
        Ok(snaps)
    }

    async fn insert_snaps(&self, snaps: &[Snap]) -> Result<(), DataAccessError> {
        log::info!("Inserting batch of {} snaps", snaps.len());
        let start_time = Instant::now();

        for snap in snaps {
            self.insert_snap(snap).await?;
        }

        let total_time = start_time.elapsed();
        log::info!(
            "Successfully inserted {} snaps. Total time: {:.2}s ({:.2}ms per snap)",
            snaps.len(),
            total_time.as_secs_f32(),
            total_time.as_millis() as f32 / snaps.len() as f32
        );
        Ok(())
    }

    async fn clear_snaps(&self) -> Result<(), DataAccessError> {
        log::info!("Clearing all snaps from Redis");
        let keys = self.get_keys_by_pattern("snap:*").await?;

        if keys.is_empty() {
            log::debug!("No snaps found to clear");
            return Ok(());
        }

        log::debug!("Deleting {} snap keys", keys.len());
        let mut conn = self.get_connection().await?;

        let _: usize = conn.del(keys).await.map_err(|e| {
            log::error!("Failed to delete snaps: {}", e);
            DataAccessError::RedisDataBaseError(e)
        })?;

        log::info!("Successfully cleared all snaps");
        Ok(())
    }
}

impl VolatileSnapRepo for RedisSnapRepo {}
