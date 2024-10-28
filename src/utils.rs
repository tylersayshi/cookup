use std::fs;

use rusqlite::{params, Connection};
use std::error::Error;

pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

#[derive(Debug)]
pub struct Recipe {
    pub name: String,
    pub instructions: String,
    pub ingredients: String,
}

#[derive(Debug)]
pub struct DBRecipe {
    pub id: i32,
    pub created_at: String,
    pub name: String,
    pub instructions: String,
    pub ingredients: String,
}

pub fn save_recipe(recipe: &Recipe) -> Result<String, String> {
    let res: Result<String, Box<dyn Error>> = {
        let conn = Connection::open("cookbook.db").unwrap();

        conn.execute(
            "INSERT INTO recipes (name, instructions, ingredients) VALUES (?1, ?2, ?3)",
            params![recipe.name, recipe.instructions, recipe.ingredients],
        )
        .unwrap();

        conn.close().unwrap();

        Ok(format!("Saved {}", recipe.name))
    };

    match res {
        Ok(res) => Ok(res),
        Err(_err) => Err("Failed to save recipe".to_string()),
    }
}
