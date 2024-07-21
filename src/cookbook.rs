use inquire::Select;
use rusqlite::Connection;

use crate::utils::DBRecipe;

pub fn cookbook() {
    let conn = Connection::open("cookbook.db").unwrap();

    let mut recipes: Vec<DBRecipe> = conn
        .prepare("SELECT * FROM recipes ORDER BY id DESC")
        .unwrap()
        .query_map([], |row| {
            Ok(DBRecipe {
                id: row.get(0).unwrap(),
                created_at: row.get(1).unwrap(),
                name: row.get(2).unwrap(),
                instructions: row.get(3).unwrap(),
                ingredients: row.get(4).unwrap(),
            })
        })
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    recipes.sort_by(|a, b| a.name.cmp(&b.name));

    let options = recipes
        .iter()
        .map(|x| format!("{}. {}", x.id.clone(), x.name.clone()))
        .collect();

    let choice = Select::new("Would you like to cook one of these recipes?", options)
        .raw_prompt()
        .unwrap();

    let full_recipe = {
        let recipe = recipes.get(choice.index).unwrap();
        format!(
            "\nRecipe: {}\n\nIngredients:\n{}\nInstructions:\n{}",
            recipe.name, recipe.ingredients, recipe.instructions
        )
    };

    print!("{}", full_recipe);
}
