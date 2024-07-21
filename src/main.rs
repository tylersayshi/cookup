use inquire::Select;
use std::fmt;
mod chatbot;
mod cookbook;
mod recipes;
mod utils;
use rusqlite::Connection;

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
    let cookbook_exists = utils::file_exists("cookbook.db");

    if !cookbook_exists {
        println!("Creating cookbook.db");
        let conn = Connection::open("cookbook.db").unwrap();

        conn.execute(
            "CREATE TABLE recipes (
                id INTEGER PRIMARY KEY,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                name TEXT NOT NULL,
                instructions TEXT NOT NULL,
                ingredients TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        conn.close().unwrap();
    }

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

// problems to address:
// Need a name per each recipe
//   - need to be able to edit the name
// need to be able to parse ingredients and instructions

// TODO: ctrl-some key to go back
