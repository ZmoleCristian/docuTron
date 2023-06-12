//Project: docuTron
//Description: A simple program that uses OpenAI's API to generate documentation for code
//Author website: https://tragdate.ninja
//Author: @tragDate on github tiktok and youtube
//git repo: https://github.com/tragDate/docuTron
//License: GPL-3.0

use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use tokio::fs;
use tokio::io::AsyncReadExt;
use async_recursion::async_recursion;

#[derive(StructOpt, Debug)]
#[structopt(name = "docuTron")]
struct Opt {
    #[structopt(short = "h", long)]
    here: bool,
    #[structopt(short = "e", long, default_value = "")]
    extensions: String,
    #[structopt(short = "m", long, default_value = "gpt-3.5-turbo")]
    model: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let api_key = dotenv::var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable must be set");
    let model = opt.model;
    let markdown = if opt.here {
        let extensions: Vec<&str> = opt.extensions.split(",").collect();
        let content = read_files_with_extension_recursive(Path::new("."), &extensions).await.unwrap();
        send_to_gpt(&api_key, &content, &model).await
    } else {
        let mut input = String::new();
        tokio::io::stdin().read_to_string(&mut input).await.unwrap();
        send_to_gpt(&api_key, &input, &model).await
    };
    println!("{}", markdown);
}

#[async_recursion]
async fn read_files_with_extension_recursive(
    dir: &Path,
    extensions: &[&str],
) -> io::Result<String> {
    let mut contents = String::new();

    if dir.is_dir() {
        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && has_valid_extension(&path, extensions) {
                contents.push_str(&read_file_content(path).await);
            } else if !extensions.is_empty() && path.is_dir() {
                let subdir_contents =
                    read_files_with_extension_recursive(&path, extensions).await?;
                if !subdir_contents.trim().is_empty() {
                    contents += &subdir_contents;
                }
            }
        }
    }

    Ok(contents)
}

async fn read_file_content(path: PathBuf) -> String {
    match fs::read_to_string(&path).await {
        Ok(file_content) => {
            format!("\n==>{}<==\n\n{}", path.to_string_lossy().as_ref(), file_content)
        }
        Err(err) => {
            eprintln!("Error reading {:?}: {:?}", path, err);
            String::new()
        }
    }
}

fn has_valid_extension(file_path: &PathBuf, valid_extensions: &[&str]) -> bool {
    file_path.extension()
        .and_then(OsStr::to_str)
        .map_or(false, |ext| {
            valid_extensions.iter().any(|valid_ext| ext == *valid_ext)
        })
}

async fn send_to_gpt(api_key: &str, input: &str, model: &str) -> String {
    let client = reqwest::Client::new();
    let system_prompt = "You are a markdown expert, that does not talk, only writes the extensive markdown documentation, just pure markdown syntax, use this format in this order: program name as title, description, features, requirements, installation build(if necessary, use newlines before codeblocks), usage(if you can asume after build/install, showcase multiple flags if necessary), author, License to make your documentation as good as possible, try to not use actual code from source code, keep in mind that your input is composed from multiple files coming from the same parent directory and all the input representes only one program, detect the programing language and understand how files call each other, do not make documentation for each file, only for the main file, the other files are just called by the main file, and they are here to aid you in your documentation, the filename name sits in this format ==> /path/to/file/filename.extension <==";
    let request_body = serde_json::json!({
        "model": format!("{}", model),
        "messages": [
            {"role": "system", "content": format!("{}", system_prompt)},
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

    let response_text: serde_json::Value = response.json().await.expect("Failed to parse GPT response");

    if response_text["error"].is_string() {
        println!("Error: {}", response_text["error"].as_str().unwrap());
    }

    response_text["choices"][0]["message"]["content"].as_str().unwrap_or("Failed to create documentation").to_string()
}
