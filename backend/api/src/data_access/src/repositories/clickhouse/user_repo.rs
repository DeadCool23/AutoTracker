use super::create_clickhouse_client;
use super::MaxIDRow;
use crate::error::DataAccessError;
use crate::repositories_traits::UserRepository;
use async_trait::async_trait;
use clickhouse::{Client, Row};
use models::{Document, Role, User};
use serde::Deserialize;

pub struct ClickHouseUserRepo {
    client: Client,
}

impl ClickHouseUserRepo {
    pub async fn from(clickhouse_url: &str) -> Result<Self, DataAccessError> {
        log::info!("Connecting to ClickHouse database for User repository");

        let client = create_clickhouse_client(clickhouse_url);

        log::info!("Successfully connected to ClickHouse");
        Ok(Self { client })
    }
}

#[allow(dead_code)]
#[derive(Debug, Row, Deserialize, Clone)]
struct ClickHouseUserRow {
    id: u32,
    login: String,
    password: String,
    role: String,
    name: String,
    surname: String,
    lastname: Option<String>,
    is_verified: u8,
    passport_serial: Option<i32>,
    passport_num: Option<i32>,
}

impl ClickHouseUserRepo {
    fn form_row_to_user(row: &ClickHouseUserRow) -> User {
        log::debug!("Converting database row to User");
        let passport_serial: Option<i32> = row.passport_serial;
        let passport_num: Option<i32> = row.passport_num;

        let user = User {
            name: row.name.clone(),
            surname: row.surname.clone(),
            lastname: row.lastname.clone(),
            email: row.login.clone(),
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
        };
        log::debug!("Converted user: {:?}", user);
        user
    }

    async fn get_row_by_email(
        &self,
        email: &str,
    ) -> Result<Option<ClickHouseUserRow>, DataAccessError> {
        log::info!("ClickHouse: Getting user by email: {}", email);

        let query = "SELECT * FROM AppUser WHERE login = ? LIMIT 1";
        log::debug!("Executing query: {}", query);

        let row = self
            .client
            .query(&query)
            .bind(email)
            .fetch_all::<ClickHouseUserRow>()
            .await
            .map_err(|e| {
                log::error!("ClickHouse query failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        if row.len() == 1 {
            log::info!("User found: {:#?}", email);
            Ok(Some(row[0].clone()))
        } else {
            log::info!("User not found: {:#?}", email);
            Ok(None)
        }
    }

    async fn gen_id(&self) -> Result<u32, DataAccessError> {
        let max_id_result = self
            .client
            .query("SELECT max(id) as max_id FROM AppUser")
            .fetch_one::<MaxIDRow>()
            .await
            .map_err(|e| {
                log::error!("Failed to get max id: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        Ok(max_id_result.max_id + 1)
    }

    async fn insert_user_with_id(
        &self,
        id: u32,
        user: &User,
        password: &str,
    ) -> Result<(), DataAccessError> {
        log::info!("ClickHouse: Inserting user: {}", user.email);

        let (passport_serial, passport_num) = match &user.passport {
            Some(p) => (
                Some(p.serial.parse::<i32>().map_err(|_| {
                    DataAccessError::InvalidInput("Invalid passport serial".into())
                })?),
                Some(p.number.parse::<i32>().map_err(|_| {
                    DataAccessError::InvalidInput("Invalid passport number".into())
                })?),
            ),
            None => (None, None),
        };

        let role_str = match user.role {
            Role::user => "user",
            Role::operator => "operator",
            Role::audit => "audit",
        };

        let query = format!(
            "INSERT INTO AppUser (id, login, password, role, name, surname, lastname, is_verified, passport_serial, passport_num)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, {}, {})",
            passport_serial.map_or("NULL".into(), |v| v.to_string()),
            passport_num.map_or("NULL".into(), |v| v.to_string())
        );

        self.client
            .query(&query)
            .bind(id)
            .bind(&user.email)
            .bind(password)
            .bind(role_str)
            .bind(&user.name)
            .bind(&user.surname)
            .bind(user.lastname.as_ref().unwrap_or(&"NULL".to_string()))
            .bind(if user.is_verified { 1 } else { 0 })
            .execute()
            .await
            .map_err(|e| {
                log::error!("ClickHouse insert failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        Ok(())
    }
}

#[async_trait]
impl UserRepository for ClickHouseUserRepo {
    async fn get_user_by_auth_info(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<User>, DataAccessError> {
        log::info!("ClickHouse: Getting user by auth info: {}", email);

        let query = "SELECT * FROM AppUser WHERE login = ? AND password = ? LIMIT 1";

        let row = self
            .client
            .query(&query)
            .bind(email)
            .bind(password)
            .fetch_all::<ClickHouseUserRow>()
            .await
            .map_err(|e| {
                log::error!("ClickHouse query failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        if row.len() == 1 {
            log::info!("User authenticated successfully: {}", email);
            Ok(Some(Self::form_row_to_user(&row[0])))
        } else {
            log::warn!("Authentication failed for user: {}", email);
            Ok(None)
        }
    }

    async fn get_user_by_passport(
        &self,
        passport: &Document,
    ) -> Result<Option<User>, DataAccessError> {
        log::info!("Getting user by passport: {:#?}", passport);
        let serial = passport.serial.parse::<i32>().map_err(|_| {
            log::error!("Invalid passport serial format: {}", passport.serial);
            DataAccessError::InvalidInput("Invalid passport serial format".to_string())
        })?;
        let number = passport.number.parse::<i32>().map_err(|_| {
            log::error!("Invalid passport number format: {}", passport.number);
            DataAccessError::InvalidInput("Invalid passport number format".to_string())
        })?;
        let query =
            "SELECT * FROM AppUser WHERE passport_serial = $1 AND passport_num = $2 LIMIT 1";
        log::debug!("Executing query: {}", query);

        let row = self
            .client
            .query(&query)
            .bind(serial)
            .bind(number)
            .fetch_all::<ClickHouseUserRow>()
            .await
            .map_err(|e| {
                log::error!("ClickHouse query failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        if row.len() == 1 {
            log::info!("User found: {:#?}", passport);
            Ok(Some(Self::form_row_to_user(&row[0])))
        } else {
            log::info!("User not found: {:#?}", passport);
            Ok(None)
        }
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, DataAccessError> {
        log::info!("ClickHouse: Getting user by email: {}", email);

        let row = self.get_row_by_email(email).await?;

        match row {
            Some(row) => {
                log::info!("User found: {}", email);
                Ok(Some(Self::form_row_to_user(&row)))
            }
            None => {
                log::info!("User not found: {}", email);
                Ok(None)
            }
        }
    }

    async fn insert_user(&self, user: &User, password: &str) -> Result<(), DataAccessError> {
        self.insert_user_with_id(self.gen_id().await?, user, password)
            .await
    }

    async fn update_user_passport(
        &self,
        email: &String,
        passport: &Document,
    ) -> Result<(), DataAccessError> {
        log::info!("Verify user with passport: {:#?}", passport);

        if self.get_row_by_email(email).await?.is_none() {
            return Err(DataAccessError::ClickHouseBaseError(
                clickhouse::error::Error::Custom("User not found".to_string()),
            ));
        }

        let serial = passport.serial.parse::<i32>().map_err(|_| {
            log::error!("Invalid passport serial format: {}", passport.serial);
            DataAccessError::InvalidInput("Invalid passport serial format".to_string())
        })?;
        let number = passport.number.parse::<i32>().map_err(|_| {
            log::error!("Invalid passport number format: {}", passport.number);
            DataAccessError::InvalidInput("Invalid passport number format".to_string())
        })?;

        let query = "
            ALTER TABLE AppUser 
            UPDATE
              passport_serial = ?,
              passport_num = ?,
              is_verified = 1
            WHERE login = ?
        ";

        self.client
            .query(query)
            .bind(serial)
            .bind(number)
            .bind(email)
            .execute()
            .await
            .map_err(|e| {
                log::error!("ClickHouse insert failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        Ok(())
    }
}

impl ClickHouseUserRepo {
    pub async fn delete_user(&self, user: &User) -> Result<(), DataAccessError> {
        log::info!("Deleting user: {}", user.email);
        let query = "ALTER TABLE AppUser DELETE WHERE login = ?";
        log::debug!("Executing delete query: {}", query);

        self.client
            .query(&query)
            .bind(&user.email)
            .execute()
            .await
            .map_err(|e| {
                log::error!("ClickHouse delete failed: {}", e);
                DataAccessError::ClickHouseBaseError(e)
            })?;

        log::info!("User deleted successfully: {}", user.email);
        Ok(())
    }
}
