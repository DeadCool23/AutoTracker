use super::{create_clickhouse_client, MaxIDRow};
use crate::error::DataAccessError;
use crate::repositories_traits::TrackInfoRepository;
use async_trait::async_trait;
use chrono::{Datelike, NaiveDate, Utc};
use clickhouse::{Client, Row};
use models::{Car, Document, Role, TrackInfo, User};
use serde::Deserialize;

pub struct ClickHouseTrackInfoRepo {
    client: Client,
}

impl ClickHouseTrackInfoRepo {
    pub async fn from(clickhouse_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Connecting to ClickHouse database for TrackInfo repository");

        let client = create_clickhouse_client(clickhouse_url);

        log::info!("Successfully connected to ClickHouse");
        Ok(Self { client })
    }
}

impl ClickHouseTrackInfoRepo {
    fn transform_mask_for_psql_like(gos_number: &str) -> String {
        log::debug!("Transforming gos number mask: {}", gos_number);
        let mut transformed_mask = gos_number.replace("*", "_");
        if let Some(last_char) = gos_number.chars().last() {
            if !last_char.is_digit(10) {
                transformed_mask.pop();
                transformed_mask.push('%');
            }
        }
        log::debug!("Transformed mask: {}", transformed_mask);
        transformed_mask
    }

    fn joined_tables_query() -> String {
        log::debug!("Generating joined tables query for track info");
        "SELECT
            formatDateTime(t.route_date, '%d.%m.%Y') as route_date, 
            formatDateTime(t.track_time, '%R %d.%m.%Y') as track_time, 
            a.name, 
            a.surname, 
            a.lastname, 
            a.login, 
            a.is_verified, 
            a.passport_serial, 
            a.passport_num, 
            a.role,
            o.name as oname, 
            o.surname as osurname, 
            o.lastname as olastname, 
            s.gos_num, 
            s.model, 
            s.mark,
            c.color, 
            formatDateTime(s.release_date, '%d.%m.%Y') as release_date, 
            s.vin,
            s.sts_serial, 
            s.sts_num, 
            p.pts_serial, 
            p.pts_number
         FROM CarOwner o
         JOIN Car c ON o.id = c.owner_id
         JOIN TrackInfo t ON t.car_id = c.id
         JOIN AppUser a ON a.id = t.user_id
         JOIN STS s ON c.id = s.car_id
         JOIN PTS p ON c.id = p.id"
            .to_string()
    }

    async fn gen_id(&self) -> Result<u32, DataAccessError> {
        let max_id_result = self
            .client
            .query("SELECT max(id) as max_id FROM TrackInfo")
            .fetch_one::<MaxIDRow>()
            .await
            .map_err(|e| {
                log::error!("Failed to get max id: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        Ok(max_id_result.max_id + 1)
    }
}

#[derive(Deserialize, Row)]
struct IdPair {
    car_id: u32,
    user_id: u32,
}

#[derive(Debug, Deserialize, Row, Clone)]
struct TrackInfoRow {
    route_date: String,
    track_time: String,

    name: String,
    surname: String,
    lastname: Option<String>,
    login: String,
    is_verified: u8,
    passport_serial: Option<i32>,
    passport_num: Option<i32>,
    role: String,

    oname: String,
    osurname: String,
    olastname: Option<String>,

    gos_num: String,
    model: String,
    mark: String,

    color: String,
    release_date: String,
    vin: String,

    sts_serial: i32,
    sts_num: i32,

    pts_serial: i32,
    pts_number: i32,
}

impl ClickHouseTrackInfoRepo {
    fn track_info_row_to_track_info(row: TrackInfoRow) -> TrackInfo {
        let passport_serial: Option<i32> = row.passport_serial;
        let passport_num: Option<i32> = row.passport_num;

        let date = NaiveDate::parse_from_str(&row.release_date, "%d.%m.%Y")
            .map_err(|e| {
                log::error!("Invalid date format: {}", e);
                DataAccessError::InvalidInput(e.to_string())
            })
            .expect("REASON");

        TrackInfo {
            track_time: row.track_time,
            route_date: row.route_date,
            car: Car {
                gos_num: row.gos_num,
                model: row.model,
                owner_fio: (row.osurname, row.oname, row.olastname),
                mark: row.mark,
                color: row.color,
                year: date.year() as u16,
                vin: row.vin,
                sts: Document {
                    serial: format!("{:0>4}", row.sts_serial.to_string()),
                    number: format!("{:0>6}", row.sts_num.to_string()),
                },
                pts: Document {
                    serial: format!("{:0>4}", row.pts_serial.to_string()),
                    number: format!("{:0>6}", row.pts_number.to_string()),
                },
            },
            user: User {
                name: row.name,
                surname: row.surname,
                lastname: row.lastname,
                email: row.login,
                role: match row.role.as_str() {
                    "user" => Role::user,
                    "operator" => Role::operator,
                    "audit" => Role::audit,
                    _ => Role::user,
                },
                is_verified: row.is_verified != 0,
                passport: match (passport_serial, passport_num) {
                    (Some(serial), Some(num)) => Some(Document {
                        serial: format!("{:0>4}", serial.to_string()),
                        number: format!("{:0>6}", num.to_string()),
                    }),
                    _ => None,
                },
            },
        }
    }

    fn track_info_rows_to_tracks_info(rows: &[TrackInfoRow]) -> Vec<TrackInfo> {
        log::debug!(
            "Converting {} database rows to TrackInfo objects",
            rows.len()
        );

        rows.into_iter()
            .map(|row| Self::track_info_row_to_track_info((*row).clone()))
            .collect()
    }
}

impl ClickHouseTrackInfoRepo {
    fn build_filter_query(
        firstname: Option<&str>,
        surname: Option<&str>,
        lastname: Option<&str>,
        passport: Option<Document>,
        gos_num_mask: Option<&str>,
        date: Option<&str>,
    ) -> Result<String, DataAccessError> {
        let mut query_builder = Self::joined_tables_query();
        query_builder.push_str(" WHERE 1 = 1 ");

        let pserial = match &passport {
            Some(psprt) => Some(psprt.serial.clone().parse::<i32>().map_err(|_| {
                log::error!("Invalid passport serial format: {}", &psprt.serial);
                DataAccessError::InvalidInput("Invalid passport serial format".to_string())
            })?),
            None => None,
        };

        let pnumber = match &passport {
            Some(psprt) => Some(psprt.number.clone().parse::<i32>().map_err(|_| {
                log::error!("Invalid passport number format: {}", &psprt.number);
                DataAccessError::InvalidInput("Invalid passport number format".to_string())
            })?),
            None => None,
        };

        if let Some(frstname) = firstname {
            let val = frstname.replace("'", "");
            query_builder.push_str(&format!(" AND a.name = '{}' ", val));
        }

        if let Some(surname) = surname {
            let val = surname.replace("'", "");
            query_builder.push_str(&format!(" AND a.surname = '{}' ", val));
        }

        if let Some(lastname) = lastname {
            let val = lastname.replace("'", "");
            query_builder.push_str(&format!(" AND a.lastname = '{}' ", val));
        }

        if let Some(pserial) = pserial {
            query_builder.push_str(&format!(" AND a.passport_serial = {} ", pserial));
        }

        if let Some(pnum) = pnumber {
            query_builder.push_str(&format!(" AND a.passport_num = {} ", pnum));
        }

        if let Some(dt) = date {
            let dt = NaiveDate::parse_from_str(dt, "%d.%m.%Y")
                .map_err(|e| {
                    log::error!("Invalid date format: {}", e);
                    DataAccessError::InvalidInput(e.to_string())
                })?
                .format("%Y-%m-%d")
                .to_string();

            let val = dt.replace("'", "");
            query_builder.push_str(&format!(" AND t.track_time::date = '{}' ", val));
        }

        if let Some(gos_num) = gos_num_mask {
            let val = gos_num.replace("'", "");
            query_builder.push_str(&format!(" AND  s.gos_num LIKE '{}' ", val));
        }

        Ok(query_builder)
    }
}

#[async_trait]
impl TrackInfoRepository for ClickHouseTrackInfoRepo {
    async fn insert_track_info(
        &self,
        gos_num: &str,
        user_login: &str,
        route_date: &str,
    ) -> Result<(), DataAccessError> {
        log::info!(
            "Inserting track info for vehicle {} by user {} on date {}",
            gos_num,
            user_login,
            route_date
        );

        let date = NaiveDate::parse_from_str(route_date, "%d.%m.%Y").map_err(|e| {
            log::error!("Invalid date format: {}", e);
            DataAccessError::InvalidInput(e.to_string())
        })?;

        let moscow_time = Utc::now() + chrono::Duration::hours(3);
        let moscow_naive = moscow_time.naive_utc();
        log::debug!("Current Moscow time: {}", moscow_naive);
        let moscow_naive = moscow_naive.format("%Y-%m-%d %H:%M:%S").to_string();

        let fid = self
            .client
            .query(
                "
                SELECT c.id as car_id, a.id as user_id
                FROM Car c
                JOIN STS s ON s.car_id = c.id
                JOIN AppUser a ON a.login = ?
                WHERE s.gos_num = ?
                ",
            )
            .bind(user_login)
            .bind(gos_num)
            .fetch_one::<IdPair>()
            .await
            .map_err(|e| {
                log::error!("Failed to fetch IDs: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        let id = self.gen_id().await?;
        let query = "
            INSERT INTO TrackInfo (id, user_id, track_time, route_date, car_id)
            VALUES (?, ?, ?, ?, ?)
        ";

        self.client
            .query(query)
            .bind(id)
            .bind(fid.user_id)
            .bind(moscow_naive)
            .bind(date)
            .bind(fid.car_id)
            .execute()
            .await
            .map_err(|e| {
                log::error!("Insert failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Successfully inserted track info");
        Ok(())
    }

    async fn get_tracks_info_by_filters(
        &self,
        firstname: Option<&str>,
        surname: Option<&str>,
        lastname: Option<&str>,
        passport: Option<Document>,
        gos_num_mask: Option<&str>,
        date: Option<&str>,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        let transformed_gos_num = gos_num_mask.map(|gsn| Self::transform_mask_for_psql_like(gsn));
        log::info!(
            "Searching cars by filters: {:?} {:?} {:?} {:?} {:?}",
            firstname,
            surname,
            lastname,
            transformed_gos_num,
            passport,
        );

        let query =
            Self::build_filter_query(firstname, surname, lastname, passport, gos_num_mask, date)?;
        log::debug!("Executing query:\n{}", query);

        let rows = self
            .client
            .query(&query)
            .fetch_all::<TrackInfoRow>()
            .await
            .map_err(|e| {
                log::error!("Query failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Found {} cars matching filters", rows.len());
        Ok(Self::track_info_rows_to_tracks_info(&rows))
    }

    async fn get_track_info_by_date(&self, date: &str) -> Result<Vec<TrackInfo>, DataAccessError> {
        log::info!("Getting track info by date: {}", date);

        let query = Self::build_filter_query(None, None, None, None, None, Some(date))?;
        log::debug!("Executing query:\n{}", query);

        let rows = self
            .client
            .query(&query)
            .fetch_all::<TrackInfoRow>()
            .await
            .map_err(|e| {
                log::error!("Query failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Found {} cars matching filters", rows.len());
        Ok(Self::track_info_rows_to_tracks_info(&rows))
    }

    async fn get_track_info_by_car_gos_number_mask(
        &self,
        gos_number: &str,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        log::info!("Searching cars by gos number mask: {}", gos_number);
        let transformed_mask = Self::transform_mask_for_psql_like(gos_number);

        let where_query = "WHERE s.gos_num LIKE ?";
        let query = &format!("{} {}", Self::joined_tables_query(), where_query);
        log::debug!(
            "Executing query: {} with param: {}",
            query,
            transformed_mask
        );

        let rows = self
            .client
            .query(query)
            .bind(transformed_mask)
            .fetch_all::<TrackInfoRow>()
            .await
            .map_err(|e| {
                log::error!("Query failed for gos number mask {}: {}", gos_number, e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Found {} cars matching mask {}", rows.len(), gos_number);
        Ok(Self::track_info_rows_to_tracks_info(&rows))
    }

    async fn get_track_info_by_user_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        log::info!(
            "Searching cars by owner FIO: {:?} {:?} {:?}",
            name,
            surname,
            last_name
        );

        let query = Self::build_filter_query(name, surname, last_name, None, None, None)?;

        let rows = self
            .client
            .query(&query)
            .fetch_all::<TrackInfoRow>()
            .await
            .map_err(|e| {
                log::error!("Query failed for FIO search: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Found {} cars matching FIO criteria", rows.len());
        Ok(Self::track_info_rows_to_tracks_info(&rows))
    }

    async fn get_track_info_by_user_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<TrackInfo>, DataAccessError> {
        log::info!(
            "Searching cars by owner passport: {}/{}",
            passport.serial,
            passport.number
        );

        let pserial = passport.serial.parse::<i32>().map_err(|_| {
            log::error!("Invalid passport serial format: {}", passport.serial);
            DataAccessError::InvalidInput("Invalid passport serial format".to_string())
        })?;
        let pnum = passport.number.parse::<i32>().map_err(|_| {
            log::error!("Invalid passport number format: {}", passport.number);
            DataAccessError::InvalidInput("Invalid passport number format".to_string())
        })?;

        let where_query = "WHERE a.passport_serial = ? AND a.passport_num = ?";
        let query = &format!("{} {}", Self::joined_tables_query(), where_query);
        log::debug!(
            "Executing query: {} with params: {}, {}",
            query,
            pserial,
            pnum
        );

        let rows = self
            .client
            .query(query)
            .bind(pserial)
            .bind(pnum)
            .fetch_all::<TrackInfoRow>()
            .await
            .map_err(|e| {
                log::error!("Query failed for passport search: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Found {} cars matching passport criteria", rows.len());
        Ok(Self::track_info_rows_to_tracks_info(&rows))
    }
}
