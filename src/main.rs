//Project: docuTron
//Description: A simple program that uses OpenAI's GPT-4 to generate documentation for code
//Author website: https:tragdate.ninja
//Author: @tragDate on github tiktok and youtube
//git repo: https://github.com/tragDate/docuTron
//License: GPL-3.0

use std::io::{self, Read};
use reqwest::Client;
use serde_json::json;
use serde_json::Value;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let api_key = dotenv::var("OPENAI_API_KEY").expect("OPENAI_API_KEY enviorment variable must be set");
    let markdown = tokio::runtime::Runtime::new().unwrap().block_on(send_to_gpt(&api_key, &input));
    println!("{}", markdown);
}

async fn send_to_gpt(api_key: &str, input: &str) -> String {
    let client = Client::new();
    let request_body = json!({
        "model": "gpt-3.5",
        "messages": [
            {"role": "system", "content": "You are a markdown expert, 
            that does not talk,
            only writes the extensive markdown documentation, 
            without markdown code block or specifing the language, 
            just pure markdown syntax,
            use this format in this order: program name as title, description, features, requirements, installation, usage, author, License
            to make your documentation as good as possible, try to not use actual code from source code,
            keep in mind that your input is composed from multiple files comeing from the same parent directory and all the input representes only one program,
            detect the programing language and understand how filles call each other,
            do not make documentation for each file, only for the main file,
            the other files are just called by the main file, and they are here to aid you in your documentation,
            the filename name sits in this format ==> /path/to/file/filename.extension <==
            "},
            {"role": "user", "content": format!("Write documentation for this code: {}", input)}
        ]
    });

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()
        .await
        .expect("Failed to send request to GPT");
    
    let response_text: Value = response.json().await.expect("Failed to parse GPT response");
    if response_text["error"].is_string() {
        println!("Error: {}", response_text["error"].as_str().unwrap());
    }
    let markdown = response_text["choices"][0]["message"]["content"].as_str().unwrap_or("Failed to create documentation").to_string();
  
    markdown
}
