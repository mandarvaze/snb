use chrono::Local;
use std::env;
use std::fs;
use std::io;
use std::process::{Command, Stdio};

pub fn get_current_timestamp() -> String {
    Local::now().format("%Y%m%d%H%M%S").to_string()
}

pub fn get_filename(filename: Option<String>, title: Option<String>) -> String {
    let extension = env::var("SNB_DEFAULT_EXTENSION").unwrap_or_else(|_| "md".to_string());
    filename.unwrap_or_else(|| {
        if let Some(title) = title {
            // Convert title to lowercase and replace spaces with underscores
            let formatted_title = title.to_lowercase().replace(' ', "_");
            format!("{}.{}", formatted_title, extension)
        } else {
            let now = get_current_timestamp();
            format!("{}.{}", now, extension)
        }
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

pub fn get_title_from_extension(title: &str, file_path: &str) -> String {
    // Extract the file extension
    let extension = file_path.split('.').last().unwrap_or("");

    // return the title based on the file extension
    match extension {
        "md" => format!("# {}", title),
        "org" => format!("* {}", title),
        "adoc" => format!("= {}", title),
        _any_other_extension => title.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_filename() {
        // Test case 1: When filename is provided
        let filename = Some("test.md".to_string());
        let title = None;
        assert_eq!(get_filename(filename, title), "test.md");

        // Test case 2: When filename is None (uses timestamp)
        let filename = None;
        let title = None;
        let result = get_filename(filename, title);
        assert!(result.ends_with(".md")); // Default extension
        assert_eq!(result.len(), 17); // YYYYMMDDHHMMSS.md = 17 chars

        // Test case 3: With custom extension from environment
        env::set_var("SNB_DEFAULT_EXTENSION", "org");
        let filename = None;
        let title = None;
        let result = get_filename(filename, title);
        assert!(result.ends_with(".org")); // Custom extension
        assert_eq!(result.len(), 18); // YYYYMMDDHHMMSS.org = 18 chars
        // Now that non-default extension is tested, remove it
        env::remove_var("SNB_DEFAULT_EXTENSION");

        // Test case 4: title is provided
        let filename = None;
        let title = Some("Test Title".to_string());
        let result = get_filename(filename, title);
        assert!(result.ends_with(".md")); // default extension
        assert_eq!(result.len(), 13); // test_title.md = 13 chars
        assert!(!result.contains(" ")); // Space is replaced by underscore
        assert!(result.contains("_")); // Space is replaced by underscore

        // Test case 5: is filename contains extension, use that extension
        let filename = Some("test.txt".to_string());
        let title = None;
        let result = get_filename(filename, title);
        println!("result: {}", result);
        assert!(result.ends_with(".txt")); // use extension from filename
        assert_eq!(result.len(), 8); // test.org = 8 chars        
    }
}
