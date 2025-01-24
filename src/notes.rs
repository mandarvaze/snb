use crate::common;
use std::fs;
use std::io::Write;

pub fn add_note(content: String, filename: Option<String>, title: Option<String>) {
    let home_dir = common::init::get_home_dir();
    let filename = common::utils::get_filename(filename);
    let full_path = home_dir.join(filename);
    println!("Adding Note with Content {}", content);
    println!("Saving to {}", full_path.display());

    let mut file = fs::File::create(&full_path).expect("Unable to create file");
    file.write_all(content.as_bytes())
        .expect("Unable to write data");
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
    let home_dir = common::init::get_home_dir();
    let entries = std::fs::read_dir(home_dir).expect("Unable to read directory");

    for entry in entries {
        let entry = entry.expect("Unable to get entry");
        let path = entry.path();

        if path.extension().map(|s| s == "md").unwrap_or(false) {
            let filename = path.file_name().unwrap().to_string_lossy();
            let first_line = std::fs::read_to_string(&path)
                .expect("Unable to read file")
                .lines()
                .next()
                .map(|line| line.to_string()) // Convert Option<&str> to Option<String>
                .unwrap_or_else(|| "".to_string());
            println!("{} - {}", filename, first_line);
        }
    }
}
