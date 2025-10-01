use super::MaxIDRow;
use crate::error::DataAccessError;
use crate::repositories_traits::SnapRepository;
use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use clickhouse::{Client, Row};
use models::{Camera, Location, Snap};
use serde::Deserialize;

use super::create_clickhouse_client;

pub struct ClickHouseSnapRepo {
    client: Client,
}

impl ClickHouseSnapRepo {
    pub async fn from(clickhouse_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Connecting to ClickHouse database for Snap repository");

        let client = create_clickhouse_client(clickhouse_url);

        log::info!("Successfully connected to ClickHouse");
        Ok(Self { client })
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Row)]
struct CarSnapshotRow {
    id: u64,
    speed: Option<u16>,
    gos_num: String,
    snap_datetime: NaiveDateTime,
    camera_id: u32,
    is_radar: bool,
    longitude: f64,
    latitude: f64,
}

impl ClickHouseSnapRepo {
    fn joined_tables_query() -> String {
        log::debug!("Generating joined tables query");
        "SELECT 
            s.id,
            s.speed,
            s.gos_num,
            s.snap_datetime,
            c.id AS camera_id,
            c.is_radar,
            c.longitude,
            c.latitude
        FROM CarSnapshot s
        INNER JOIN Camera c ON s.camera_id = c.id"
            .to_string()
    }

    fn from_rows_to_snaps(rows: &[CarSnapshotRow]) -> Vec<Snap> {
        log::debug!("Converting {} database rows to Snap objects", rows.len());
        let snaps = rows
            .into_iter()
            .map(|r| Snap {
                camera: Camera {
                    id: r.camera_id as usize,
                    is_radar: r.is_radar,
                    location: Location {
                        longitude: r.longitude,
                        latitude: r.latitude,
                    },
                },
                speed: r.speed,
                gos_num: r.gos_num.clone(),
                date: r.snap_datetime.date().format("%d.%m.%Y").to_string(),
                time: r.snap_datetime.time().format("%H:%M").to_string(),
            })
            .collect();

        snaps
    }

    async fn gen_id(&self) -> Result<u32, DataAccessError> {
        let max_id_result = self
            .client
            .query("SELECT max(id) as max_id FROM CarSnapshot")
            .fetch_one::<MaxIDRow>()
            .await
            .map_err(|e| {
                log::error!("Failed to get max id: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        Ok(max_id_result.max_id + 1)
    }
}

#[async_trait]
impl SnapRepository for ClickHouseSnapRepo {
    async fn insert_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        log::info!(
            "Inserting snap for vehicle {} at {} {}",
            snap.gos_num,
            snap.date,
            snap.time
        );

        let naive_dt = NaiveDateTime::parse_from_str(
            &format!("{} {}", snap.date, snap.time),
            "%d.%m.%Y %H:%M",
        )
        .map_err(|e| {
            log::error!("Failed to parse datetime: {}", e);
            DataAccessError::InvalidInput(e.to_string())
        })?;

        let datetime_str = naive_dt.format("%Y-%m-%d %H:%M:%S").to_string();

        let id = self.gen_id().await?;
        let query = &format!(
            "
                INSERT INTO CarSnapshot (id, camera_id, snap_datetime, speed, gos_num, road_line)
                VALUES (?, ?, toDateTime('{}'), {}, ?, 0)
            ",
            datetime_str,
            if snap.speed.is_some() {
                snap.speed.unwrap().to_string()
            } else {
                "NULL".to_string()
            }
        );

        self.client
            .query(query)
            .bind(id)
            .bind(snap.camera.id as u32)
            .bind(&snap.gos_num)
            .execute()
            .await
            .map_err(|e| {
                log::error!("Insert failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Successfully inserted snap with id {}", id);
        Ok(())
    }

    async fn get_car_snaps_by_date(
        &self,
        gos_number: &str,
        date: &str,
    ) -> Result<Vec<Snap>, DataAccessError> {
        log::info!("Getting snaps for vehicle {} on date {}", gos_number, date);

        let date = NaiveDate::parse_from_str(date, "%d.%m.%Y").map_err(|e| {
            log::error!("Invalid date format: {}", e);
            DataAccessError::InvalidInput(e.to_string())
        })?;

        let start_datetime = date.and_hms_opt(0, 0, 0).unwrap();
        let end_datetime = date.and_hms_opt(23, 59, 59).unwrap();

        let start = start_datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        let end = end_datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        let query = format!(
            "
            {}
            WHERE s.gos_num = ? 
              AND s.snap_datetime BETWEEN toDateTime('{}') AND toDateTime('{}')
            ORDER BY s.snap_datetime
            ",
            Self::joined_tables_query(),
            start,
            end,
        );

        let rows = self
            .client
            .query(&query)
            .bind(gos_number)
            .fetch_all::<CarSnapshotRow>()
            .await
            .map_err(|e| {
                log::error!("Query failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::debug!("Found {} matching snaps", rows.len());

        Ok(Self::from_rows_to_snaps(&rows))
    }
}

impl ClickHouseSnapRepo {
    pub async fn delete_snap(&self, snap: &Snap) -> Result<(), DataAccessError> {
        log::info!(
            "Deleting snap for vehicle {} at {} {}",
            snap.gos_num,
            snap.date,
            snap.time
        );

        let datetime = NaiveDateTime::parse_from_str(
            &format!("{} {}", snap.date, snap.time),
            "%d.%m.%Y %H:%M",
        )
        .map_err(|e| {
            log::error!("Failed to parse datetime: {}", e);
            DataAccessError::InvalidInput(e.to_string())
        })?;

        let query = format!(
            "ALTER TABLE CarSnapshot DELETE WHERE camera_id = ? AND snap_datetime = toDateTime('{}') AND gos_num = ?",
            datetime.format("%Y-%m-%d %H:%M:%S"),
        );

        self.client
            .query(&query)
            .bind(snap.camera.id)
            .bind(&snap.gos_num)
            .execute()
            .await
            .map_err(|e| {
                log::error!("Failed to delete snap: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Delete query submitted (mutation scheduled)");
        Ok(())
    }
}
