use crate::read_cookbook::read_cookbook;
use inquire::Select;

pub fn cookbook() {
    let recipes = read_cookbook();

    let options = recipes
        .iter()
        .enumerate()
        .map(|(i, x)| format!("{}. {}", i.clone() + 1, x.name.clone()))
        .collect();

    let choice = Select::new("Would you like to cook one of these recipes?", options)
        .raw_prompt()
        .unwrap();

    let full_recipe = {
        let recipe = recipes.get(choice.index).unwrap();
        format!(
            "\n Created at: {}\n\nRecipe: {}\n\nIngredients:\n{}\nInstructions:\n{}",
            recipe.created_at, recipe.name, recipe.ingredients, recipe.instructions
        )
    };

    print!("{}", full_recipe);
}
