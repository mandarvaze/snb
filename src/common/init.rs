use std::fs;
use std::path::Path;

/// Initializes the snb home directory, if it doesn't already exist.
///
/// The home directory is created at ~/.snb/home.
///
/// If the directory already exists, a message indicating as such is printed.
pub fn init() {
    let home_dir = home::home_dir().unwrap().join(".snb/home");

    if !Path::new(&home_dir).exists() {
        fs::create_dir_all(&home_dir).expect("Failed to create directory");
        // println!("Created directory: {:?}", home_dir);
    } /* else {
          println!("Directory already exists: {:?}", home_dir);
      } */
}
