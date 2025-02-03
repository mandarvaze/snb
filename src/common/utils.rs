use chrono::Local;
use std::env;
use std::fs;
use std::io;
use std::process::{Command, Stdio};

pub fn get_current_timestamp() -> String {
    Local::now().format("%Y%m%d%H%M%S").to_string()
}

pub fn get_filename(filename: Option<String>) -> String {
    let extension = env::var("SNB_DEFAULT_EXTENSION").unwrap_or_else(|_| "md".to_string());
    filename.unwrap_or_else(|| {
        let now = get_current_timestamp();
        format!("{}.{}", now, extension)
    })
}

/// Delete the given file from filesystem
/// Replace the entry from .index file with empty line
/// Return an error if the file cannot be removed
pub fn delete_file(filename: &str) -> io::Result<()> {
    let full_path = crate::common::init::get_home_dir().join(filename);
    fs::remove_file(full_path)
        // Replace the entry from .index file with empty line
        .and_then(|_| crate::common::index::remove_filename_from_index(filename))
}

/// Open the given file for editing in editor specified by
/// either EDITOR or VISUAL environment variable
/// if neither is set, "vi" is used
/// Returns an error if the file cannot be opened or written to.
pub fn edit_file(filename: &str) -> io::Result<()> {
    let full_path = crate::common::init::get_home_dir().join(filename);

    let editor = env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".to_string());
    let mut cmd = Command::new(editor.clone());
    cmd.arg(full_path.to_str().unwrap());
    cmd.stdin(Stdio::inherit()) // Important: Inherit stdin for interactive editors
        .stdout(Stdio::inherit()) // Important: Inherit stdout for interactive editors
        .stderr(Stdio::inherit()); // Important: Inherit stderr for interactive editors

    // Execute the command and wait for it to finish.
    let status = cmd.status()?;
    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to run editor: {}", editor),
        ));
    }

    Ok(())
}
