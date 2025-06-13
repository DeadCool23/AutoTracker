use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub struct Finder<'a> {
    filename: &'a Path,
}

impl<'a> Finder<'a> {
    pub fn new() -> Self {
        Finder {
            filename: Path::new(".env"),
        }
    }

    pub fn filename(mut self, filename: &'a Path) -> Self {
        self.filename = filename;
        self
    }

    pub fn find(self) -> Result<PathBuf, io::Error> {
        Ok(find(&env::current_dir()?, self.filename)?)
    }
}

pub fn find(directory: &Path, filename: &Path) -> Result<PathBuf, io::Error> {
    let candidate = directory.join(filename);

    match fs::metadata(&candidate) {
        Ok(metadata) => {
            if metadata.is_file() {
                return Ok(candidate);
            }
        }
        Err(error) => {
            if error.kind() != io::ErrorKind::NotFound {
                return Err(error);
            }
        }
    }

    if let Some(parent) = directory.parent() {
        find(parent, filename)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "path not found"))
    }
}

pub fn from_filename<P: AsRef<Path>>(filename: P) -> Result<PathBuf, io::Error> {
    Ok(Finder::new().filename(filename.as_ref()).find()?)
}
