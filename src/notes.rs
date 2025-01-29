use crate::common;
use crate::common::index::{add_filename_to_index, get_index_file_path};
use std::fs;

pub fn add_note(
    content: String,
    filename: Option<String>,
    title: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = common::init::get_home_dir();
    let filename = common::utils::get_filename(filename);
    let full_path = home_dir.join(filename.clone()); // Clone filename for index

    // Create content with title if provided
    let content = if let Some(title) = title {
        format!("# {}\n\n{}", title, content)
    } else {
        content
    };

    println!("Adding Note to {}", full_path.display());

    fs::write(&full_path, content)?;

    add_filename_to_index(&filename)?;

    Ok(())
}

pub fn view_note(id: &u32) {
    println!("Viewing Note {}", id);
}

pub fn delete_note(id: &u32) {
    println!("Deleting Note {}", id);
}

pub fn edit_note(id: &u32) {
    println!("Editing Note {}", id);
}

pub fn list_notes() {
    // Read from the .index file
    let index_file_path = get_index_file_path();
    let index_content = fs::read_to_string(index_file_path).expect("Unable to read index file");

    // Process each line in the index file
    for (line_number, line) in index_content.lines().enumerate() {
        let filename = line.trim(); // Assuming each line contains a filename
        let full_path = common::init::get_home_dir().join(filename);
        let first_line = std::fs::read_to_string(&full_path)
            .expect("Unable to read file")
            .lines()
            .next()
            .map(|line| line.to_string()) // Convert Option<&str> to Option<String>
            .unwrap_or_default();
        println!("[{}] {} - {}", line_number + 1, filename, first_line);
    }
}
