use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Confirm, Select, Text};
use reqwest::Client;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::utils::Recipe;

#[derive(Serialize)]
struct ChatGPTRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize, serde::Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatGPTResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

fn parse_recipe(recipe: &str, name: &str) -> Result<Recipe, String> {
    let ingredients_ind = recipe.find("Ingredients:").unwrap();
    let instructions_ind = recipe.find("Instructions:").unwrap();

    let ingredients =
        recipe[(ingredients_ind + "Ingredients:".len() + 1)..instructions_ind - 1].to_string();
    let instructions = recipe[(instructions_ind + "Instructions:".len() + 1)..].to_string();

    if instructions == "" || ingredients == "" {
        return Err(format!("Invalid recipe\n {}", recipe));
    }

    Ok(Recipe {
        name: name.to_string(),
        instructions,
        ingredients,
    })
}

async fn send_request(api_key: &str, prompt: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let model = "gpt-3.5-turbo"; // Specify the model you want to use

    let messages = vec![Message {
        role: "user".to_string(),
        content: prompt.to_string(),
    }];

    let request_body = ChatGPTRequest {
        model: model.to_string(),
        messages,
    };

    let progress_bar = Arc::new(Mutex::new(ProgressBar::new(60)));
    let progress_bar_clone = Arc::clone(&progress_bar);

    progress_bar.lock().await.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {msg}")?
            .progress_chars("#>-"),
    );

    let api_key_clone = api_key.to_string();

    let request_task = tokio::spawn(async move {
        // Make the HTTP GET request
        client
            .post(url)
            .header("Authorization", format!("Bearer {}", &api_key_clone))
            .json(&request_body)
            .send()
            .await
            .expect("Failed ChatGPT request")
    });

    let progress_task = tokio::spawn(async move {
        let pb = progress_bar_clone.lock().await;
        for _i in 0..60 {
            pb.inc(1);
            sleep(Duration::from_millis(50)).await; // Simulate progress update
        }
        pb.finish_with_message("Done?");
    });

    // Wait for both tasks to complete
    let (response, _progress) = tokio::join!(request_task, progress_task);

    Ok(response
        .unwrap()
        .json::<ChatGPTResponse>()
        .await
        .unwrap()
        .choices[0]
        .message
        .content
        .clone())
}

async fn complete_prompt(
    prompt_start: &str,
    api_key: &str,
    question: &str,
) -> Result<String, Box<dyn Error>> {
    let answer = Text::new(question).prompt().unwrap();

    let prompt = format!("{}. {}", &prompt_start, &answer);

    let response = send_request(&api_key, &prompt).await.unwrap();

    Ok(response)
}

async fn random_list(prompt_start: &str, token: &str) -> Result<String, Box<dyn Error>> {
    let prompt = format!("{}. This list should be random and unique.", prompt_start);

    let response = send_request(&token, &prompt).await.unwrap();

    Ok(response)
}

fn save_recipe(recipe: &Recipe) -> Result<String, String> {
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

fn do_you_want_to_save(recipe: &Recipe) -> Result<String, String> {
    let ans = Confirm::new("Do you want to save this recipe? (Y/n)").prompt();

    match ans {
        Ok(true) => save_recipe(recipe),
        Ok(false) => Ok("Not saved".to_string()),
        Err(err) => Err(err.to_string()),
    }
}

pub async fn chatbot() {
    let api_key =
        env::var("OPENAI_API_KEY").expect("Please set the OPENAI_API_KEY environment variable.");

    let mut prompts = HashMap::new();
    prompts.insert("One Recipe", "Write a recipe for the following dish:");

    prompts.insert(
        "List of Recipes",
        "Please suggest some recipe ideas for a new dish.",
    );

    let choice = Select::new(
        "Which of the following would you like help with?",
        prompts.keys().cloned().collect(),
    )
    .prompt()
    .unwrap();

    let recipe_result: Result<(Recipe, String), String> = match choice {
        "One Recipe" => {
            let name = prompts.get("One Recipe").unwrap();

            let res = complete_prompt(name, &api_key, "Describe your dish:")
                .await
                .expect("Failed to generate recipe");

            let recipe = parse_recipe(&res, name).unwrap();

            Ok((recipe, res))
        }
        "List of Recipes" => {
            let dissatisfied = "None of these - Give me a new list!";

            let mut current_choice = dissatisfied.to_string();

            while current_choice == dissatisfied {
                let list_type = Select::new(
                    "Which type of list would you like?",
                    vec!["Random", "I have some ideas"],
                )
                .prompt()
                .unwrap();

                let chat_response = match list_type {
                    "I have some ideas" => {
                        complete_prompt(
                            prompts.get("List of Recipes").unwrap(),
                            &api_key,
                            "What are your ideas?",
                        )
                        .await
                    }
                    "Random" => {
                        random_list(prompts.get("List of Recipes").unwrap(), &api_key).await
                    }
                    _ => panic!("Invalid list type"),
                }
                .unwrap();

                let mut recipe_ideas: Vec<_> = chat_response
                    .split("\n")
                    .filter(|x| !x.is_empty())
                    .map(|x| x.to_string())
                    .collect();

                recipe_ideas.push("None of these - Give me a new list!".to_string());

                current_choice = Select::new(
                    "Which recipe would you like to choose?",
                    recipe_ideas.to_vec(),
                )
                .prompt()
                .unwrap();
            }

            let res = send_request(
                &api_key,
                &format!("please write a recipe for {}", current_choice),
            )
            .await
            .expect("Failed to generate recipe");

            let name = current_choice.split(". ").nth(1).unwrap();
            let recipe = parse_recipe(&res, name).unwrap();

            Ok((recipe, res))
        }
        _ => Err("Invalid choice".to_string()),
    };

    match recipe_result {
        Ok((parsed_recipe, full_recipe)) => {
            println!("Recipe:\n\n{}", full_recipe);
            let save_msg = do_you_want_to_save(&parsed_recipe).unwrap();
            println!("{}", save_msg);
        }
        Err(err) => println!("Error: {}", err),
    }

    // Send the request and print the response
}
