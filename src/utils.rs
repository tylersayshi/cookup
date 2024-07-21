use std::fs;

pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

#[derive(Debug)]
pub struct Recipe {
    pub name: String,
    pub instructions: String,
    pub ingredients: String,
}
