use super::create_clickhouse_client;
use crate::error::DataAccessError;
use crate::repositories_traits::CarRepository;
use async_trait::async_trait;
use chrono::{Datelike, NaiveDate};
use clickhouse::{Client, Row};
use models::{Car, Document};
use serde::Deserialize;

pub struct ClickHouseCarRepo {
    client: Client,
}

impl ClickHouseCarRepo {
    pub async fn from(clickhouse_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Connecting to ClickHouse database for Car repository");

        let client = create_clickhouse_client(clickhouse_url);

        log::info!("Successfully connected to ClickHouse");
        Ok(Self { client })
    }
}

impl ClickHouseCarRepo {
    fn transform_mask_for_psql_like(gos_number: &str) -> String {
        log::debug!("Transforming mask for PostgreSQL LIKE: {}", gos_number);
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
        log::debug!("Generating joined tables query for cars");
        "SELECT 
            o.name, 
            o.surname, 
            o.lastname, 
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
         JOIN STS s ON c.id = s.car_id
         JOIN PTS p ON c.id = p.id"
            .to_string()
    }
}

#[derive(Debug, Deserialize, Row, Clone)]
struct CarRow {
    name: String,
    surname: String,
    lastname: Option<String>,
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

impl ClickHouseCarRepo {
    fn car_row_to_car(row: CarRow) -> Car {
        let date = NaiveDate::parse_from_str(&row.release_date, "%d.%m.%Y")
            .map_err(|e| {
                log::error!("Invalid date format: {}", e);
                DataAccessError::InvalidInput(e.to_string())
            })
            .expect("REASON");

        Car {
            gos_num: row.gos_num,
            model: row.model,
            owner_fio: (row.surname, row.name, row.lastname),
            mark: row.mark,
            color: row.color,
            year: date.year() as u16,
            vin: row.vin,
            sts: Document {
                serial: format!("{:04}", row.sts_serial),
                number: format!("{:06}", row.sts_num),
            },
            pts: Document {
                serial: format!("{:04}", row.pts_serial),
                number: format!("{:06}", row.pts_number),
            },
        }
    }

    fn car_rows_to_cars(rows: &[CarRow]) -> Vec<Car> {
        log::debug!("Converting {} database rows to Car objects", rows.len());

        rows.into_iter()
            .map(|row| Self::car_row_to_car((*row).clone()))
            .collect()
    }
}

impl ClickHouseCarRepo {
    fn build_filter_query(
        firstname: Option<&str>,
        surname: Option<&str>,
        lastname: Option<&str>,
        passport: Option<Document>,
        gos_num_mask: Option<&str>,
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
            query_builder.push_str(&format!(" AND o.name = '{}' ", val));
        }

        if let Some(surname) = surname {
            let val = surname.replace("'", "");
            query_builder.push_str(&format!(" AND o.surname = '{}' ", val));
        }

        if let Some(lastname) = lastname {
            let val = lastname.replace("'", "");
            query_builder.push_str(&format!(" AND o.lastname = '{}' ", val));
        }

        if let Some(pserial) = pserial {
            query_builder.push_str(&format!(" AND o.passport_serial = {} ", pserial));
        }

        if let Some(pnum) = pnumber {
            query_builder.push_str(&format!(" AND o.passport_num = {} ", pnum));
        }

        if let Some(gos_num) = gos_num_mask {
            let val = gos_num.replace("'", "");
            query_builder.push_str(&format!(" AND  s.gos_num LIKE '{}' ", val));
        }

        Ok(query_builder)
    }
}

#[async_trait]
impl CarRepository for ClickHouseCarRepo {
    async fn get_cars_by_filters(
        &self,
        firstname: Option<&str>,
        surname: Option<&str>,
        lastname: Option<&str>,
        passport: Option<Document>,
        gos_num_mask: Option<&str>,
    ) -> Result<Vec<Car>, DataAccessError> {
        let transformed_gos_num = gos_num_mask.map(|gsn| Self::transform_mask_for_psql_like(gsn));
        log::info!(
            "Searching cars by filters: {:?} {:?} {:?} {:?} {:?}",
            firstname,
            surname,
            lastname,
            transformed_gos_num,
            passport,
        );

        let query = Self::build_filter_query(firstname, surname, lastname, passport, gos_num_mask)?;
        log::debug!("Executing query:\n{}", query);

        let rows = self
            .client
            .query(&query)
            .fetch_all::<CarRow>()
            .await
            .map_err(|e| {
                log::error!("Query failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Found {} cars matching filters", rows.len());
        Ok(Self::car_rows_to_cars(&rows))
    }

    async fn get_car_by_gos_number_mask(
        &self,
        gos_number: &str,
    ) -> Result<Vec<Car>, DataAccessError> {
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
            .fetch_all::<CarRow>()
            .await
            .map_err(|e| {
                log::error!("Query failed for gos number mask {}: {}", gos_number, e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Found {} cars matching mask {}", rows.len(), gos_number);
        Ok(Self::car_rows_to_cars(&rows))
    }

    async fn get_car_by_owner_fio(
        &self,
        name: Option<&str>,
        surname: Option<&str>,
        last_name: Option<&str>,
    ) -> Result<Vec<Car>, DataAccessError> {
        log::info!(
            "Searching cars by owner FIO: {:?} {:?} {:?}",
            name,
            surname,
            last_name
        );

        let query = Self::build_filter_query(name, surname, last_name, None, None)?;

        let rows = self
            .client
            .query(&query)
            .fetch_all::<CarRow>()
            .await
            .map_err(|e| {
                log::error!("Query failed for FIO search: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Found {} cars matching FIO criteria", rows.len());
        Ok(Self::car_rows_to_cars(&rows))
    }

    async fn get_car_by_owner_passport(
        &self,
        passport: &Document,
    ) -> Result<Vec<Car>, DataAccessError> {
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

        let where_query = "WHERE o.passport_serial = ? AND o.passport_num = ?";
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
            .fetch_all::<CarRow>()
            .await
            .map_err(|e| {
                log::error!("Query failed for passport search: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("Found {} cars matching passport criteria", rows.len());
        Ok(Self::car_rows_to_cars(&rows))
    }
}
