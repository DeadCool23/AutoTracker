use lazy_static::lazy_static;

#[allow(unused_imports)]
use data_access::repositories::clickhouse::*;
#[allow(unused_imports)]
use data_access::repositories::mocked::*;
#[allow(unused_imports)]
use data_access::repositories::postgres::*;
#[allow(unused_imports)]
use data_access::repositories::redis::*;
#[allow(unused_imports)]
use data_access::repositories::tandem::*;
#[allow(unused_imports)]
use data_access::repositories_traits::*;

#[allow(unused_macros)]
macro_rules! connect_repository {
    ($reconnect_manager:expr, $repo_type:ty, $from_fn:path, $url:expr) => {
        match $from_fn($url).await {
            Ok(repo) => Ok(repo),
            Err(_) => match $reconnect_manager.reconnect().await {
                Ok(_) => $from_fn($url).await,
                Err(e) => Err(e),
            },
        }
    };
}

macro_rules! select_repository {
    (
        $db:expr, 
        $postgres_repo_type:ty, 
        $clickhouse_repo_type:ty, 
        $output_enum:path
    ) => {
        match $db {
            AvailableDB::Postgres => {
                let repo = match <$postgres_repo_type>::from(&PG_URL).await {
                    Ok(repo) => repo,
                    Err(e) => {
                        log::error!("Can't connect to postgres: {}", e);
                        return None;
                    }
                };
                log::debug!("Getted {} repo", stringify!($postgres_repo_type));
                $output_enum(Box::new(repo))
            }
            AvailableDB::ClickHouse => {
                let repo = match <$clickhouse_repo_type>::from(&CLICKHOUSE_URL).await {
                    Ok(repo) => repo,
                    Err(e) => {
                        log::error!("Can't connect to clickhouse: {}", e);
                        return None;
                    }
                };
                log::debug!("Getted {} repo", stringify!($clickhouse_repo_type));
                $output_enum(Box::new(repo))
            }
        }
    };
}

const MAX_RETRIES: usize = 5;
const RETRY_DELAY_SECS: usize = 5;
const TANDEM_THRESHOLD: usize = 10;

lazy_static! {
    static ref PG_CONN_MANAGER: PgConnectionManager =
        PgConnectionManager::new(PG_URL.to_string(), MAX_RETRIES, RETRY_DELAY_SECS);
}

#[derive(PartialEq)]
enum AvailableDB {
    Postgres,
    ClickHouse,
}

impl AvailableDB {
    fn from_str(db_str: String) -> Result<AvailableDB, ()> {
        match db_str.as_str() {
            "postgres" => Ok(AvailableDB::Postgres),
            "clickhouse" => Ok(AvailableDB::ClickHouse),
            _ => Err(()),
        }
    }
}

pub enum DARepos {
    UserRepo(Box<dyn UserRepository>),
    SnapRepo(Box<dyn SnapRepository>),
    CameraRepo(Box<dyn CameraRepository>),
    CarRepo(Box<dyn CarRepository>),
    TrackInfoRepo(Box<dyn TrackInfoRepository>),
}

#[allow(non_camel_case_types)]
pub struct DATA_ACCESSES;

impl DATA_ACCESSES {
    pub async fn get(name: &str) -> Option<DARepos> {
        let db = AvailableDB::from_str(cfg::var("vars.main_db"))
            .expect("Incorrect DB var. Avalible DB: postgres, clickhouse");

        match name {
            "snap_repo" => {
                let snap_repo = match db {
                    AvailableDB::Postgres => {
                        let psql_snap_repo = match PgSnapRepo::from(&PG_URL).await {
                            Ok(repo) => repo,
                            Err(e) => {
                                log::error!("Can't connect to redis: {}", e);
                                return None;
                            }
                        };
                        log::debug!("Getted PgSnapRepo");

                        let redis_snap_repo = match RedisSnapRepo::from(&REDIS_URL) {
                            Ok(repo) => repo,
                            Err(e) => {
                                log::error!("Can't connect to redis: {}", e);
                                return None;
                            }
                        };
                        log::debug!("Getted RedisSnapRepo");

                        let tandem_snap_repo = TandemSnapRepo::from(
                            Box::new(psql_snap_repo),
                            Box::new(redis_snap_repo),
                        )
                        .with_threshold(TANDEM_THRESHOLD);
                        log::debug!(
                            "Formed TandemSnapRepo where:
                                - main_storage: PgSnapRepo
                                - extra_storage: RedisSnapRepo
                        "
                        );

                        DARepos::SnapRepo(Box::new(tandem_snap_repo))
                    }
                    AvailableDB::ClickHouse => {
                        select_repository!(
                            AvailableDB::ClickHouse,
                            PgSnapRepo,
                            ClickHouseSnapRepo,
                            DARepos::SnapRepo
                        )
                    }
                };

                log::info!("Sending SnapRepository");
                Some(snap_repo)
            }
            "user_repo" => {
                let res = select_repository!(db, PgUserRepo, ClickHouseUserRepo, DARepos::UserRepo);

                log::info!("Sending UserRepository");
                Some(res)
            }
            "camera_repo" => {
                let res =
                    select_repository!(db, PgCameraRepo, ClickHouseCameraRepo, DARepos::CameraRepo);

                log::info!("Sending CameraRepository");
                Some(res)
            }
            "car_repo" => {
                let res = select_repository!(db, PgCarRepo, ClickHouseCarRepo, DARepos::CarRepo);

                log::info!("Sending CarRepository");
                Some(res)
            }
            "track_info_repo" => {
                let res = select_repository!(
                    db,
                    PgTrackInfoRepo,
                    ClickHouseTrackInfoRepo,
                    DARepos::TrackInfoRepo
                );

                log::info!("Sending TrackInfoRepository");
                Some(res)
            }
            _ => {
                log::error!("Incorrect data access key");
                panic!("Incorrect data acces key");
            }
        }
    }
}
