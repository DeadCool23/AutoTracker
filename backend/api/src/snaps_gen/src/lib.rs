use chrono::Local;
use data_access::{
    models::{Camera, Snap},
    repositories_traits::{CameraRepository, CarRepository},
};
use di_container::{DARepos, DATA_ACCESSES};
use rand::{Rng, seq::IndexedRandom};

const MAX_SPEED: u16 = 120;

async fn get_rand_camera<Rand: Rng>(
    rng: &mut Rand,
    repo: Box<dyn CameraRepository>,
) -> Option<Camera> {
    let cnt = repo.get_camera_count().await.unwrap_or(0);
    if cnt == 0 {
        log::warn!("No camera in database");
        return None;
    }

    let cam_id = rng.random_range(1..=cnt);
    if let Ok(cam) = repo.get_camera_by_id(cam_id).await {
        log::info!("Getted camera with id: {cam_id}");
        return Some(cam);
    }
    None
}

async fn get_rand_gos_num_from_db<Rand: Rng>(
    rng: &mut Rand,
    repo: Box<dyn CarRepository>,
) -> Option<String> {
    let cars = match repo.get_car_by_gos_number_mask("*******").await {
        Ok(cars) => cars,
        _ => return None,
    };
    if let Some(rand_car) = cars.choose(rng) {
        log::info!("Getted car with gos_num: {}", rand_car.gos_num);
        Some(rand_car.gos_num.clone())
    } else {
        None
    }
}

pub async fn gen_snap<Rand: Rng>(rng: &mut Rand) -> Option<Snap> {
    let car_repo = match DATA_ACCESSES::get("car_repo").await {
        Some(DARepos::CarRepo(repo)) => repo,
        _ => return None,
    };

    let camera_repo = match DATA_ACCESSES::get("camera_repo").await {
        Some(DARepos::CameraRepo(repo)) => repo,
        _ => return None,
    };

    let camera = match get_rand_camera(rng, camera_repo).await {
        Some(c) => c,
        _ => {
            log::error!("Can't get random camera");
            return None;
        }
    };

    let gos_num = match get_rand_gos_num_from_db(rng, car_repo).await {
        Some(gn) => gn,
        _ => {
            log::error!("Can't get random gos_num");
            return None;
        }
    };

    let now = Local::now();

    let time = now.format("%H:%M").to_string();
    let date = now.format("%d.%m.%Y").to_string();

    let speed = if camera.is_radar {
        Some(rng.random_range(40..=MAX_SPEED))
    } else {
        None
    };

    let snap = Snap {
        speed,
        camera,
        time,
        date,
        gos_num,
    };
    log::info!("Generated snap: {:#?}", snap);
    Some(snap)
}

pub async fn gen_snaps<Rand: Rng>(cnt: usize, rng: &mut Rand) -> Vec<Snap> {
    let mut snaps = vec![];

    for _ in 0..cnt {
        if let Some(snap) = gen_snap(rng).await {
            snaps.push(snap);
        } else {
            break;
        }
    }

    log::info!("Generated {} snaps: {:#?}", snaps.len(), snaps);
    snaps
}
