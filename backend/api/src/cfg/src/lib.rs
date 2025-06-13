mod finder;

use config::{Config, File, FileFormat};
use lazy_static::lazy_static;
use std::{fmt::Display, path::PathBuf};

pub use finder::from_filename;

lazy_static! {
    static ref CFG_FILENAME: String = "config.cfg".to_string();
    static ref CFG_PATH: PathBuf = from_filename(&*CFG_FILENAME).expect(&*CFG_FILENAME);
    static ref CFG: Config = {
        Config::builder()
            .add_source(File::from(CFG_PATH.clone()).format(FileFormat::Ini))
            .build()
            .unwrap_or_else(|e| {
                let info = format!("{}: {e:?}", &*CFG_FILENAME);
                log::error!("{info}");
                panic!("{info}")
            })
    };
}

pub fn var<T: ToString + Display>(key: T) -> String {
    CFG.get_string(&key.to_string()).unwrap_or_else(|e| {
        let info = format!("{}: {e:?}", *CFG_FILENAME);
        log::error!("{info}");
        panic!("{info}")
    })
}
