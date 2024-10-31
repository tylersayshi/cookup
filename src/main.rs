use inquire::Select;
use std::fmt;
mod chatbot;
mod cookbook;
mod read_cookbook;
mod utils;
mod write_recipe;
use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Received Ctrl+C, exiting...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    while running.load(Ordering::SeqCst) {
        let options = vec![Choice::Cookbook, Choice::Chatbot, Choice::NewRecipe];

        let choice = Select::new("What would you like to cook up today?", options).prompt();

        match choice {
            Ok(selection) => match selection {
                Choice::Cookbook => {
                    cookbook::cookbook();
                }
                Choice::Chatbot => {
                    chatbot::chatbot().await;
                }
                Choice::NewRecipe => {
                    write_recipe::recipes();
                }
            },
            Err(inquire::error::InquireError::OperationInterrupted) => {
                // nothing to do
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }

        break;
    }

    println!("Goodbye!");
}

// problems to address:
//  - editable recipes
//  - parse/validate ingredients and instructions

// TODO: ctrl-some key to go back
