use anyhow::{Context, Result};
use dotenv::dotenv;
use reqwest::multipart::{Form, Part};
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get API key from environment
    let api_key = env::var("ALLDEBRID_API_KEY")
        .context("Failed to get ALLDEBRID_API_KEY from environment. Please set it in .env file")?;
    
    // Get link from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} LINK [PASSWORD]", args[0]);
        return Ok(());
    }
    
    let link = &args[1];
    
    // Optional password
    let password = if args.len() > 2 {
        args[2].clone()
    } else {
        String::from("") // Empty password if not provided
    };
    
    // Create a client
    let client = reqwest::Client::new();
    
    // Prepare the form data
    let mut form = Form::new().part("link", Part::text(link.clone()));
    
    // Add password if not empty
    if !password.is_empty() {
        form = form.part("password", Part::text(password));
    }
    
    // Make the API request
    let response = client
        .post("http://api.alldebrid.com/v4/link/unlock")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await
        .context("Failed to send request to AllDebrid API")?;
    
    // Parse response as JSON
    let response_json: Value = response
        .json()
        .await
        .context("Failed to parse API response as JSON")?;
    
    // Check if request was successful
    if response_json["status"] == "success" {
        // Extract and print only the "link" field
        if let Some(link) = response_json["data"]["link"].as_str() {
            println!("{}", link);
        } else {
            eprintln!("Link field not found in response");
        }
    } else {
        // Print error message if request failed
        eprintln!("API request failed: {:?}", response_json);
    }
    
    Ok(())
}
