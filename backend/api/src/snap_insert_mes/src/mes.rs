use data_access::repositories::postgres::{PG_URL, PgSnapRepo};
use lazy_static::lazy_static;
use rand::SeedableRng;
use rand::rngs::StdRng;
use snaps_gen::gen_snaps;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum InsertType {
    ByOne,
    ByValues,
    ByCopy,
}

pub const MES_CNT: usize = 5;
lazy_static! {
    pub static ref MES_DIR: String = "measures".to_string();
    pub static ref MES_FILES: HashMap<InsertType, String> = {
        let mut map = HashMap::new();
        map.insert(InsertType::ByOne, "by_one.csv".to_string());
        map.insert(InsertType::ByValues, "by_values.csv".to_string());
        map.insert(InsertType::ByCopy, "by_copy.csv".to_string());
        map
    };
}

#[allow(unreachable_patterns)]
pub async fn get_insert_measure(
    insert_cnt: usize,
    insert_type: InsertType,
    repo: &PgSnapRepo,
) -> f64 {
    let mut rng = StdRng::from_os_rng();
    let snaps = gen_snaps(insert_cnt, &mut rng).await;
    let mut total = 0.;

    for i in 0..MES_CNT {
        log::info!("Getting {i} mesure for insert {:#?}", insert_type);
        let time = match insert_type {
            InsertType::ByOne => repo.insert_snaps_by_one(&snaps).await,
            InsertType::ByValues => repo.insert_snaps_by_values(&snaps).await,
            InsertType::ByCopy => repo.insert_snaps_by_copy(&snaps).await,
            _ => panic!("Undefined insert type"),
        }
        .unwrap();
        repo.delete_snaps(&snaps).await.unwrap();
        total += time.as_nanos() as f64;
    }

    let mes = total / MES_CNT as f64;
    log::info!("Average mes for insert {:#?}: {}", insert_type, mes);
    mes
}

#[allow(unreachable_patterns)]
async fn warmup_inserts(repo: &PgSnapRepo, insert_type: InsertType) -> Result<(), String> {
    let mut rng = StdRng::from_os_rng();
    let snaps = gen_snaps(1, &mut rng).await;

    let insert_result = match insert_type {
        InsertType::ByOne => repo.insert_snaps_by_one(&snaps).await,
        InsertType::ByValues => repo.insert_snaps_by_values(&snaps).await,
        InsertType::ByCopy => repo.insert_snaps_by_copy(&snaps).await,
        _ => return Err("Undefined insert type".to_string()),
    };

    insert_result.map_err(|e| format!("Insert failed: {}", e))?;
    repo.delete_snaps(&snaps)
        .await
        .map_err(|e| format!("Delete failed: {}", e))?;

    Ok(())
}

async fn measure_insert_duration(
    insert_cnt: usize,
    insert_type: InsertType,
    repo: &PgSnapRepo,
) -> f64 {
    log::info!("Measuring insert for {} snaps", insert_cnt);
    get_insert_measure(insert_cnt, insert_type, repo).await
}

pub async fn get_insert_measures(insert_cnts: &[usize], insert_type: InsertType) -> Vec<f64> {
    let snaps_repo = match PgSnapRepo::from(&PG_URL).await {
        Ok(repo) => repo,
        Err(e) => {
            log::error!("Failed to connect PgSnapRepo: {}", e);
            return Vec::new();
        }
    };

    if let Err(e) = warmup_inserts(&snaps_repo, insert_type).await {
        log::error!("Warmup failed: {}", e);
        return Vec::new();
    }

    let mut results = Vec::with_capacity(insert_cnts.len());
    for &insert_cnt in insert_cnts {
        let duration = measure_insert_duration(insert_cnt, insert_type, &snaps_repo).await;
        log::info!("Insert count {}: {:.3} sec", insert_cnt, duration);
        results.push(duration);
    }

    results
}
