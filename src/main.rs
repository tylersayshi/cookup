use inquire::Select;
use std::fmt;
mod chatbot;
mod cookbook;
mod recipes;

enum Choice {
    Cookbook,
    Chatbot,
    NewRecipe,
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Choice::Chatbot => write!(f, "From some chatbot"),
            Choice::Cookbook => write!(f, "From the cookbook"),
            Choice::NewRecipe => write!(f, "I'm gonna write a recipe myself"),
        }
    }
}

#[tokio::main]
async fn main() {
    let options = vec![Choice::Cookbook, Choice::Chatbot, Choice::NewRecipe];

    let choice = Select::new("What would you like to cookup today?", options)
        .prompt()
        .unwrap();

    match choice {
        Choice::Cookbook => {
            cookbook::cookbook();
        }
        Choice::Chatbot => {
            chatbot::chatbot().await;
        }
        Choice::NewRecipe => {
            recipes::recipes();
        }
    }
}
