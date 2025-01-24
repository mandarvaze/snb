use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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
pub fn init() {
    let home_dir = get_home_dir();

    if !Path::new(&home_dir).exists() {
        fs::create_dir_all(&home_dir).expect("Failed to create directory");
        println!("Created directory: {:?}", home_dir);
    } /* else {
          println!("Directory already exists: {:?}", home_dir);
      }*/
}
