use inquire::{Editor, Text};

use crate::utils::Recipe;

pub fn recipes() {
    let answer = Text::new("Name of the recipe: ").prompt().unwrap();
    let ingredients = Editor::new("Ingredients (bulleted list):\n")
        .prompt()
        .unwrap();
    // TODO validate ingredients
    // TODO hold recipes in more parser friendly format for being able to play with ratios and timing
    let instructions = Editor::new("Instructions (numbered list): ")
        .prompt()
        .unwrap();

    let recipe = Recipe {
        name: answer,
        ingredients,
        instructions,
    };
    crate::utils::save_recipe(&recipe).unwrap();
}
