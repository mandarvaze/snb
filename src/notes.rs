use crate::common;
use crate::common::index::{add_filename_to_index, get_index_file_path};
use crate::common::log::debug_log;
use crate::common::utils::{delete_file, edit_file, get_simple_title, get_title_from_extension};
use clap_verbosity_flag::VerbosityFilter;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

pub fn add_note(
    content: String,
    filename: Option<String>,
    title: Option<String>,
    verbosity: VerbosityFilter,
) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = common::init::get_home_dir();
    let file_name = common::utils::get_filename(filename, title.clone());
    let full_path = home_dir.join(file_name.clone()); // Clone filename for index

    // Create content with title if provided
    let content = if let Some(title) = title {
        format!(
            "{}\n\n{}",
            get_title_from_extension(&title, &file_name),
            content
        )
    } else {
        content
    };

    debug_log(
        &verbosity,
        &format!("Adding Note to {}", full_path.display()),
    );

    fs::write(&full_path, content)?;

    add_filename_to_index(&file_name)?;

    Ok(())
}

pub fn view_note(id: &u32) {
    println!("Viewing Note {}", id);
}

pub fn delete_note(
    id: &usize,
    verbosity: VerbosityFilter,
) -> Result<(), Box<dyn std::error::Error>> {
    let filename = get_filename_from_index(id)?;
    debug_log(&verbosity, &format!("Deleting Note {}", &filename));
    delete_file(&filename)?;
    Ok(())
}

fn get_filename_from_index(id: &usize) -> Result<String, Box<dyn std::error::Error>> {
    let index_file_path = crate::common::index::get_index_file_path();
    let file = File::open(index_file_path)?;
    let reader = io::BufReader::new(file);

    // Get the nth line (filename) from the index file
    let filename = reader
        .lines()
        .nth(*id - 1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Filename not found"))??;

    Ok(filename)
}

pub fn edit_note(id: &usize, verbosity: VerbosityFilter) -> Result<(), Box<dyn std::error::Error>> {
    debug_log(&verbosity, &format!("Editing Note {}", id));

    let filename = get_filename_from_index(id)?;
    debug_log(&verbosity, &format!("Filename: {}", filename));

    edit_file(&filename)?;
    Ok(())
}

pub fn list_notes() {
    // Read from the .index file
    let index_file_path = get_index_file_path();
    let index_content = fs::read_to_string(index_file_path).expect("Unable to read index file");

    // Process each line in the index file
    for (line_number, line) in index_content.lines().enumerate() {
        let filename = line.trim(); // Assuming each line contains a filename

        if filename.is_empty() {
            // if the filename is empty, skip it
            continue;
        }
        let full_path = common::init::get_home_dir().join(filename);
        let first_line = std::fs::read_to_string(&full_path)
            .expect("Unable to read file")
            .lines()
            .next()
            .map(|line| line.to_string()) // Convert Option<&str> to Option<String>
            .unwrap_or_default();
        let title = get_simple_title(&first_line, filename);
        println!("[{:3}] {} - {}", line_number + 1, filename, title);
    }
}
