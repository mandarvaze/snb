use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::common::log::debug_log;
use clap_verbosity_flag::VerbosityFilter;

pub fn get_home_dir() -> PathBuf {
    let default_dir = format!("{}/.snb", home::home_dir().unwrap().to_str().unwrap());
    let base_dir = env::var("SNB_DIR").unwrap_or(default_dir);
    Path::new(&base_dir).join("home")
}

/// Initializes the snb home directory, if it doesn't already exist.
///
/// The home directory is created at ~/.snb/home.
///
/// If the directory already exists, a message indicating as such is printed.
///
/// Inside the home directory, empty `.index` file is created, if it doesn't already exist.
///
pub fn init(verbosity: VerbosityFilter) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = get_home_dir();

    if !Path::new(&home_dir).exists() {
        fs::create_dir_all(&home_dir)?;
        debug_log(&verbosity, &format!("Created directory: {:?}", home_dir));
    } else {
        debug_log(
            &verbosity,
            &format!("Directory already exists: {:?}", home_dir),
        );
    }

    let index_file = home_dir.join(".index");
    if !Path::new(&index_file).exists() {
        fs::File::create(&index_file)?;
        debug_log(&verbosity, &format!("Created index file: {:?}", index_file));
    } else {
        debug_log(&verbosity, ".index file already exists");
    }
    Ok(())
}
