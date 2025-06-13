use rand::rngs::StdRng;
use rand::SeedableRng;
use snaps_gen::gen_snap;

use di_container::{DARepos, DATA_ACCESSES};

const THREADS_CNT: usize = 20;
const DURATION_TIME: u64 = 2;

#[tokio::main]
async fn main() {
    logger::init(
        &format!(
            "{}/{}",
            cfg::var("logs.logs_dir"),
            cfg::var("logs.camera_inserts_log")
        ),
        true,
    );

    let mut handles = vec![];

    for _ in 0..THREADS_CNT {
        let snap_repo = match DATA_ACCESSES::get("snap_repo").await {
            Some(DARepos::SnapRepo(repo)) => repo,
            _ => panic!("Can't get snap repository"),
        };

        let mut rng = StdRng::from_rng(&mut rand::rng());

        let handle = tokio::spawn(async move {
            loop {
                if let Some(snap) = gen_snap(&mut rng).await {
                    if let Err(e) = snap_repo.insert_snap(&snap).await {
                        log::error!("Failed to save snap: {}", e);
                    } else {
                        log::info!("Successfully saved snap: {:?}", snap);
                    }
                } else {
                    log::warn!("Failed to generate snap");
                }

                tokio::time::sleep(tokio::time::Duration::from_secs(DURATION_TIME)).await;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
