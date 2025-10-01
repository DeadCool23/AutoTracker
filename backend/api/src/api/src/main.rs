mod handlers;
mod paths;
mod router;

#[tokio::main]
async fn main() {
    logger::init(
        &format!("{}/{}", cfg::var("logs.logs_dir"), cfg::var("logs.api_log")),
        true,
    );

    let app = router::init();

    let addr = cfg::var("server.api_url");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    log::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
