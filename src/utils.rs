use chrono;
use std::fs;

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

const FILE_PATH: &str = "./cookbook.ts";

pub fn save_recipe(recipe: &Recipe) -> Result<String, String> {
    // Read the existing TypeScript code
    let mut source_code = fs::read_to_string(FILE_PATH).expect("Failed to read file");

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
    fs::write(FILE_PATH, source_code).expect("Failed to write updated code");

    Ok("Saved recipe".to_string())
}
