use chrono::Local;
use data_access::{
    models::{Camera, Snap},
    repositories_traits::{CameraRepository, CarRepository},
};
use di_container::{DataContainer, Repositories};
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

async fn get_repo_car() -> Option<Box<dyn CarRepository>> {
    match DataContainer::get("car_repo").await {
        Some(Repositories::CarRepo(repo)) => Some(repo),
        _ => {
            log::error!("Can't get CarRepository");
            None
        }
    }
}

async fn get_repo_camera() -> Option<Box<dyn CameraRepository>> {
    match DataContainer::get("camera_repo").await {
        Some(Repositories::CameraRepo(repo)) => Some(repo),
        _ => {
            log::error!("Can't get CameraRepository");
            None
        }
    }
}

async fn get_random_camera_safe<R: Rng>(
    rng: &mut R,
    camera_repo: Box<dyn CameraRepository>,
) -> Option<Camera> {
    match get_rand_camera(rng, camera_repo).await {
        Some(c) => Some(c),
        None => {
            log::error!("Can't get random camera");
            None
        }
    }
}

async fn get_random_gos_num_safe<R: Rng>(
    rng: &mut R,
    car_repo: Box<dyn CarRepository>,
) -> Option<String> {
    match get_rand_gos_num_from_db(rng, car_repo).await {
        Some(gn) => Some(gn),
        None => {
            log::error!("Can't get random gos_num");
            None
        }
    }
}

fn current_time_and_date() -> (String, String) {
    let now = Local::now();
    let time = now.format("%H:%M").to_string();
    let date = now.format("%d.%m.%Y").to_string();
    (time, date)
}

fn generate_speed_if_radar<R: Rng>(rng: &mut R, is_radar: bool) -> Option<u16> {
    if is_radar {
        Some(rng.random_range(40..=MAX_SPEED))
    } else {
        None
    }
}

pub async fn gen_snap<Rand: Rng>(rng: &mut Rand) -> Option<Snap> {
    let car_repo = get_repo_car().await?;
    let camera_repo = get_repo_camera().await?;

    let camera = get_random_camera_safe(rng, camera_repo).await?;
    let gos_num = get_random_gos_num_safe(rng, car_repo).await?;

    let (time, date) = current_time_and_date();
    let speed = generate_speed_if_radar(rng, camera.is_radar);

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
