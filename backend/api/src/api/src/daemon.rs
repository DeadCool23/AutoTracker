use daemonize::Daemonize;
use std::{
    fs::{self, File},
    path::Path,
};

const NAME: &'static str = "autotracker_server";

pub async fn init() -> Daemonize<()> {
    let daemon_dir = &format!("./{}.d", NAME);

    if !Path::new(daemon_dir).exists() {
        fs::create_dir_all(daemon_dir)
            .unwrap_or_else(|_| panic!("Failed to create directory: {}", daemon_dir));
    }

    let stdout = File::create(format!("{}/{}.out", daemon_dir, NAME))
        .unwrap_or_else(|_| panic!("Failed to create stdout file in {}", daemon_dir));

    let stderr = File::create(format!("{}/{}.err", daemon_dir, NAME))
        .unwrap_or_else(|_| panic!("Failed to create stderr file in {}", daemon_dir));

    Daemonize::new()
        .pid_file(&format!("{}.pid", NAME))
        .working_directory(daemon_dir)
        .stdout(stdout)
        .stderr(stderr)
}
