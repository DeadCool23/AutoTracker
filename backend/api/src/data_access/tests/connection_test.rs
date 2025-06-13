use data_access::repositories::{clickhouse::CLICKHOUSE_URL, postgres::PG_URL, redis::REDIS_URL};
use sqlx::PgPool;

#[tokio::test]
async fn test_pg_connection() {
    let pool = PgPool::connect(&PG_URL).await;
    assert!(pool.is_ok());

    let row: Result<(i32,), sqlx::Error> = sqlx::query_as("SELECT 1")
        .fetch_one(&pool.expect("REASON"))
        .await;

    println!("{:?}", row);
    assert!(row.is_ok())
}

#[tokio::test]
async fn test_redis_connection() {
    let pool = redis::Client::open(REDIS_URL.to_string());
    assert!(pool.is_ok());
}

#[tokio::test]
async fn test_clickhouse_connection() {
    let url = url::Url::parse(&CLICKHOUSE_URL).expect("Invalid ClickHouse URL");

    let host = format!(
        "{}://{}:{}",
        url.scheme(),
        url.host_str().unwrap(),
        url.port().unwrap_or(8123)
    );

    let user = url.username();
    let password = url.password().unwrap_or("");
    let database = url.path().trim_start_matches('/');

    let pool = clickhouse::Client::default()
        .with_url(&host)
        .with_user(user)
        .with_password(password)
        .with_database(database);

    let row = pool.query("SELECT 1").fetch_all::<u8>().await;

    println!("{:?}", row);
    assert!(row.is_ok());
}
