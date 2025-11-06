use ansi_term::Colour;
use chrono_tz::Europe::Moscow;
use env_logger::Builder;
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

mod loki;
use loki::LokiLogger;

pub fn init(log_filename: &String, is_in_stdout: bool, loki_url: Option<String>) {
    let loki_logger = if let Some(l_url) = loki_url {
        let initial_labels = HashMap::from([
            ("app".to_string(), "autotracker".to_string()),
            ("tag".to_string(), cfg::var("logs.loki_app_tag")),
        ]);

        Some(LokiLogger::new(
            &format!("{}/loki/api/v1/push", l_url),
            Some(initial_labels),
        ))
    } else {
        None
    };

    let path = Path::new(log_filename);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect(&format!(
            "Failed to create log directory: {}",
            parent.display()
        ));
    }

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .expect(&format!("Can't open {}", path.display()));

    Builder::from_default_env()
        .format(move |buf, record| {
            let timestamp = chrono::Utc::now()
                .with_timezone(&Moscow)
                .format("%Y-%m-%dT%H:%M:%SZ%:z");

            let level = match record.level() {
                log::Level::Error => Colour::Red.paint(record.level().to_string()),
                log::Level::Warn => Colour::Yellow.paint(record.level().to_string()),
                log::Level::Info => Colour::Green.paint(record.level().to_string()),
                log::Level::Debug => Colour::Blue.paint(record.level().to_string()),
                log::Level::Trace => Colour::Purple.paint(record.level().to_string()),
            };

            let log_line = format!(
                "[{} {} {}] {}",
                timestamp,
                level,
                record.module_path().unwrap_or_default(),
                record.args()
            );

            writeln!(
                &log_file,
                "[{} {} {}] {}",
                timestamp,
                record.level(),
                record.module_path().unwrap_or_default(),
                record.args()
            )
            .expect("Failed to write to log file");

            if is_in_stdout {
                writeln!(buf, "{}", log_line)?
            }

            loki_logger.as_ref().map(|l| l.log_record(record));

            Ok(())
        })
        .init();
}
