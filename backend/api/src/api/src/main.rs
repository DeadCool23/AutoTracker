mod daemon;
mod handlers;
mod paths;
mod router;

use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(about = "AutoTracker Server")]
struct Cli {
    /// Run as daemon
    #[arg(
        short = 'd',
        long = "daemon",
        default_value_t = false,
        action = ArgAction::SetTrue,
    )]
    daemon: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.daemon {
        let daemonize = daemon::init().await;

        match daemonize.start() {
            Ok(_) => {
                start_server().await;
            }
            Err(e) => {
                eprintln!("Error starting daemon: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        start_server().await;
    }
}

async fn start_server() {
    logger::init(
        &format!("{}/{}", cfg::var("logs.logs_dir"), cfg::var("logs.api_log")),
        true,
    );

    let app = router::init();

    let addr = cfg::var("server.api_url");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind to address: {}\ne: {}", addr, e));

    log::info!(
        "Server started, listening on {}",
        listener.local_addr().unwrap()
    );
    log::info!("Process PID: {}", std::process::id());

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| panic!("Server error: {}", e));
}
