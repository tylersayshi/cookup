use chrono;
use std::{env, fs, path::PathBuf};

#[derive(Debug)]
pub struct Recipe {
    pub name: String,
    pub instructions: String,
    pub ingredients: String,
}

#[derive(Debug)]
pub struct SavedRecipe {
    pub created_at: String,
    pub name: String,
    pub instructions: String,
    pub ingredients: String,
}

const FILE_NAME: &str = "cookbook.ts";

fn resolve_executable_path() -> Option<PathBuf> {
    let exe_path = env::current_exe().ok()?;
    fs::canonicalize(exe_path).ok()
}

pub fn get_storage_path() -> Result<String, String> {
    let exe_dir = resolve_executable_path()
        .and_then(|path| path.parent().map(|parent| parent.to_path_buf()))
        .expect("Failed to determine executable directory");
    let storage_dir = exe_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("Failed to navigate upwards");
    let file_path = storage_dir.join(FILE_NAME);
    Ok(file_path.to_str().unwrap().to_string())
}

pub fn save_recipe(recipe: &Recipe) -> Result<String, String> {
    // Read the existing TypeScript code
    let file_path = get_storage_path().unwrap();
    let mut source_code = fs::read_to_string(&file_path).expect("Failed to read file");

    let created_at_iso = chrono::offset::Local::now().to_rfc3339()[0..10].to_string();

    // Create a new entry
    let new_entry = format!(
        r#"  {{
    name: "{name}",
    ingredients: `{ingredients}`,
    instructions: `{instructions}`, 
    created_at: "{created_at_iso}",
  }},"#,
        name = recipe.name,
        ingredients = recipe.ingredients,
        instructions = recipe.instructions,
        created_at_iso = created_at_iso,
    );

    // Insert the new entry into the default export array
    source_code = source_code.replace(
        "] satisfies Recipe[];",
        &format!("{}\n] satisfies Recipe[];", new_entry),
    );

    // Write the updated source code back to the original file
    fs::write(&file_path, source_code).expect("Failed to write updated code");

    Ok("Saved recipe".to_string())
}
