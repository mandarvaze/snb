use chrono::Local;
use std::env;

pub fn get_current_timestamp() -> String {
    Local::now().format("%Y%m%d%H%M%S").to_string()
}

pub fn get_filename(filename: Option<String>) -> String {
    let extension = env::var("SNB_DEFAULT_EXTENSION").unwrap_or_else(|_| "md".to_string());
    let filename = filename.unwrap_or_else(|| {
        let now = get_current_timestamp();
        format!("{}.{}", now, extension)
    });
    return filename;
}
