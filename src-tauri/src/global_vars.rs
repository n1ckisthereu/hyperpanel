use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Mutex;

pub static CONFIG_DIR: Lazy<Mutex<PathBuf>> = Lazy::new(|| Mutex::new(PathBuf::new()));
