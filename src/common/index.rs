use std::fs;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

pub fn get_index_file_path() -> PathBuf {
    let home_dir = crate::common::init::get_home_dir();
    home_dir.join(".index")
}

pub fn add_filename_to_index(filename: &str) -> io::Result<()> {
    let index_file_path = get_index_file_path();
    let mut index_file = fs::OpenOptions::new().append(true).open(index_file_path)?;
    writeln!(index_file, "{}", filename)?;
    Ok(())
}

pub fn remove_filename_from_index(filename: &str) -> io::Result<()> {
    let index_file_path = get_index_file_path();
    let index_file = fs::File::open(index_file_path.clone())?;
    let reader = io::BufReader::new(index_file);

    let mut temp_lines = Vec::new();
    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim() != filename.trim() {
            temp_lines.push(line);
        } else {
            // Replace the removed line with an empty line
            temp_lines.push("".to_string());
        }
    }

    let mut index_file = fs::File::create(index_file_path)?;
    for line in temp_lines {
        writeln!(index_file, "{}", line)?;
    }
    Ok(())
}
