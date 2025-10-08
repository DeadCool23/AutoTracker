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

#[allow(dead_code)]
const MAX_RETRIES: usize = 5;
#[allow(dead_code)]
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

pub enum Repositories {
    UserRepo(Box<dyn UserRepository>),
    SnapRepo(Box<dyn SnapRepository>),
    CameraRepo(Box<dyn CameraRepository>),
    CarRepo(Box<dyn CarRepository>),
    TrackInfoRepo(Box<dyn TrackInfoRepository>),
}

pub struct DataContainer;

impl DataContainer {
    pub async fn get(name: &str) -> Option<Repositories> {
        let db = AvailableDB::from_str(cfg::var("vars.main_db"))
            .expect("Incorrect DB var. Avalible DB: postgres, clickhouse");

        match name {
            "snap_repo" => Self::build_snap_repo(db).await,
            "user_repo" => Self::build_user_repo(db).await,
            "camera_repo" => Self::build_camera_repo(db).await,
            "car_repo" => Self::build_car_repo(db).await,
            "track_info_repo" => Self::build_track_info_repo(db).await,
            _ => {
                log::error!("Incorrect data access key: {}", name);
                None
            }
        }
    }

    async fn connect_volatile_pg_snap_repo() -> Option<PgSnapRepo> {
        match PgSnapRepo::from(&PG_URL).await {
            Ok(repo) => {
                log::debug!("Connected to PgSnapRepo");
                Some(repo)
            }
            Err(e) => {
                log::error!("Can't connect to Postgres SnapRepo: {}", e);
                None
            }
        }
    }

    fn connect_volatile_redis_snap_repo() -> Option<RedisSnapRepo> {
        match RedisSnapRepo::from(&REDIS_URL) {
            Ok(repo) => {
                log::debug!("Connected to RedisSnapRepo");
                Some(repo)
            }
            Err(e) => {
                log::error!("Can't connect to Redis SnapRepo: {}", e);
                None
            }
        }
    }

    fn create_tandem_snap_repo(psql_repo: PgSnapRepo, redis_repo: RedisSnapRepo) -> TandemSnapRepo {
        let tandem = TandemSnapRepo::from(Box::new(psql_repo), Box::new(redis_repo))
            .with_threshold(TANDEM_THRESHOLD);

        log::debug!("Formed TandemSnapRepo (main: PgSnapRepo, extra: RedisSnapRepo)");

        tandem
    }

    async fn build_postgres_snap_repo() -> Option<Repositories> {
        let psql_repo = Self::connect_volatile_pg_snap_repo().await?;
        log::debug!("Getted PgSnapRepo");

        let redis_repo = Self::connect_volatile_redis_snap_repo()?;
        log::debug!("Getted RedisSnapRepo");

        let tandem = Self::create_tandem_snap_repo(psql_repo, redis_repo);

        Some(Repositories::SnapRepo(Box::new(tandem)))
    }

    async fn build_clickhouse_snap_repo() -> Option<Repositories> {
        let repo = select_repository!(
            AvailableDB::ClickHouse,
            PgSnapRepo,
            ClickHouseSnapRepo,
            Repositories::SnapRepo
        );

        Some(repo)
    }

    async fn build_snap_repo(db: AvailableDB) -> Option<Repositories> {
        let repo = match db {
            AvailableDB::Postgres => Self::build_postgres_snap_repo().await?,
            AvailableDB::ClickHouse => Self::build_clickhouse_snap_repo().await?,
        };

        log::info!("Sending SnapRepository");
        Some(repo)
    }

    async fn build_user_repo(db: AvailableDB) -> Option<Repositories> {
        let repo = select_repository!(db, PgUserRepo, ClickHouseUserRepo, Repositories::UserRepo);
        log::info!("Sending UserRepository");
        Some(repo)
    }

    async fn build_camera_repo(db: AvailableDB) -> Option<Repositories> {
        let repo = select_repository!(
            db,
            PgCameraRepo,
            ClickHouseCameraRepo,
            Repositories::CameraRepo
        );
        log::info!("Sending CameraRepository");
        Some(repo)
    }

    async fn build_car_repo(db: AvailableDB) -> Option<Repositories> {
        let repo = select_repository!(db, PgCarRepo, ClickHouseCarRepo, Repositories::CarRepo);
        log::info!("Sending CarRepository");
        Some(repo)
    }

    async fn build_track_info_repo(db: AvailableDB) -> Option<Repositories> {
        let repo = select_repository!(
            db,
            PgTrackInfoRepo,
            ClickHouseTrackInfoRepo,
            Repositories::TrackInfoRepo
        );
        log::info!("Sending TrackInfoRepository");
        Some(repo)
    }
}
