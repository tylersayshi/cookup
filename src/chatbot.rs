use inquire::{Select, Text};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;

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

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let response_json: ChatGPTResponse = response.json().await?;

    Ok(response_json.choices[0].message.content.clone())
}

async fn complete_prompt(
    prompt_start: &str,
    api_key: &str,
    question: &str,
) -> Result<String, Box<dyn Error>> {
    let answer = Text::new(question).prompt().unwrap();

    let prompt = format!("{}. {}", &prompt_start, &answer);

    let response = send_request(&api_key, &prompt).await.unwrap();
    println!("Response: {}", response);

    Ok(response)
}

async fn random_list(prompt_start: &str, token: &str) -> Result<String, Box<dyn Error>> {
    let prompt = format!("{}. This list should be random and unique.", prompt_start);

    let response = send_request(&token, &prompt).await.unwrap();
    println!("From ChatGPT:\n\n {}", response);

    Ok(response)
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

    match choice {
        "One Recipe" => {
            complete_prompt(
                prompts.get("One Recipe").unwrap(),
                &api_key,
                "Describe your dish:",
            )
            .await
            .expect("Failed to generate recipe");
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

            // After the list is generated, prompt the user choose one from the list for generating a recipe
        }
        _ => {
            println!("Invalid choice");
        }
    }

    // Send the request and print the response
}
