use lazy_static::lazy_static;

use clickhouse::Client;
use url::Url;

mod camera_repo;
mod car_repo;
mod snap_repo;
mod track_info_repo;
mod user_repo;

pub use camera_repo::ClickHouseCameraRepo;
pub use car_repo::ClickHouseCarRepo;
pub use snap_repo::ClickHouseSnapRepo;
pub use track_info_repo::ClickHouseTrackInfoRepo;
pub use user_repo::ClickHouseUserRepo;

lazy_static! {
    pub static ref CLICKHOUSE_URL: String = cfg::var("database.clickhouse_url");
}

fn create_clickhouse_client(dsn: &str) -> Client {
    let url = Url::parse(dsn).expect("Invalid ClickHouse URL");

    let host = format!(
        "{}://{}:{}",
        url.scheme(),
        url.host_str().unwrap(),
        url.port().unwrap_or(8123)
    );

    let user = url.username();
    let password = url.password().unwrap_or("");
    let database = url.path().trim_start_matches('/');

    Client::default()
        .with_url(&host)
        .with_user(user)
        .with_password(password)
        .with_database(database)
}

use clickhouse::Row;
use serde::Deserialize;

#[derive(Debug, Deserialize, Row)]
struct MaxIDRow {
    max_id: u32,
}
