mod csv;
mod mes;
mod plot;

use csv::{read_from_csv, write_to_csv};
use lazy_static::lazy_static;
use mes::{InsertType, MES_DIR, MES_FILES, get_insert_measures};
use plot::{PLOT_FILENAME, plot_graphs, plot_to_htmlfile, show_plot};
use std::{fs, path::Path};

lazy_static! {
    pub static ref INSERT_SIZES: Vec<usize> =
        std::iter::once(1).chain((5..=30).step_by(5)).collect();
}

fn build_paths() -> (std::path::PathBuf, Vec<(InsertType, std::path::PathBuf)>) {
    let base = Path::new(&*MES_DIR).to_path_buf();
    let paths = vec![
        (
            InsertType::ByOne,
            base.join(MES_FILES.get(&InsertType::ByOne).unwrap()),
        ),
        (
            InsertType::ByValues,
            base.join(MES_FILES.get(&InsertType::ByValues).unwrap()),
        ),
        (
            InsertType::ByCopy,
            base.join(MES_FILES.get(&InsertType::ByCopy).unwrap()),
        ),
    ];
    (base, paths)
}

async fn get_or_load_measurements(insert_type: InsertType, file_path: &Path) -> Vec<f64> {
    if file_path.exists() {
        log::info!(
            "Getting mesurements for {:#?} from file {}",
            insert_type,
            file_path.to_str().unwrap()
        );
        read_from_csv(file_path.to_str().unwrap()).1
    } else {
        log::info!("Getting mesurments");
        let res = get_insert_measures(&INSERT_SIZES, insert_type).await;
        log::info!(
            "Writing mesurments for {:#?} to file {}",
            insert_type,
            file_path.to_str().unwrap()
        );
        write_to_csv(file_path.to_str().unwrap(), &INSERT_SIZES, &res);
        res
    }
}

#[tokio::main]
async fn main() {
    logger::init(
        &format!(
            "{}/{}",
            cfg::var("logs.logs_dir"),
            cfg::var("logs.snap_insert_mes_log")
        ),
        false,
    );

    let (dir_path, insert_paths) = build_paths();
    if !dir_path.exists() {
        log::info!("Creating dir: {}", dir_path.to_str().unwrap());
        fs::create_dir(&dir_path).unwrap();
    }

    let line_names = vec!["BY VALUE", "VALUES", "COPY"]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

    let mut inserts_mes = Vec::new();
    for (insert_type, path) in insert_paths {
        log::info!("Getting mes for insert type: {:#?}", insert_type);
        let data = get_or_load_measurements(insert_type, &path).await;
        inserts_mes.push(data);
    }

    log::info!("Drawing plot");
    let plot = plot_graphs("Замеры времени", &line_names, &INSERT_SIZES, &inserts_mes);
    log::info!("Showing plot");
    show_plot(&plot);
    log::info!("Saving plot into html: {}", &*PLOT_FILENAME);
    plot_to_htmlfile(dir_path.join(&*PLOT_FILENAME).to_str().unwrap(), &plot);
}
