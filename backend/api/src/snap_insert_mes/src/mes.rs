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
pub async fn get_insert_measures(insert_cnts: &[usize], insert_type: InsertType) -> Vec<f64> {
    let snaps_repo = PgSnapRepo::from(&PG_URL).await.unwrap();
    let mut res = Vec::with_capacity(insert_cnts.len());

    {
        let mut rng = StdRng::from_os_rng();
        let snaps = gen_snaps(1, &mut rng).await;

        let _ = match insert_type {
            InsertType::ByOne => snaps_repo.insert_snaps_by_one(&snaps).await,
            InsertType::ByValues => snaps_repo.insert_snaps_by_values(&snaps).await,
            InsertType::ByCopy => snaps_repo.insert_snaps_by_copy(&snaps).await,
            _ => panic!("Undefined insert type"),
        }
        .unwrap();
        snaps_repo.delete_snaps(&snaps).await.unwrap();
    }

    for &insert_cnt in insert_cnts {
        log::info!("Getting mesures for {} insert count", insert_cnt);
        let duration = get_insert_measure(insert_cnt, insert_type, &snaps_repo).await;
        res.push(duration);
        println!("{}: {}", insert_cnt, duration);
    }

    res
}
