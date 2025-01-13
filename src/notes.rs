use crate::common;

pub fn add_note(content: String, filename: Option<String>, title: Option<String>) {
    let filename = common::utils::get_filename(filename);
    println!("Adding Note with Content {}", content);
    println!("Saving to {}", filename);
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
    println!("Listing Notes");
}
