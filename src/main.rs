use anyhow::{Context, Result};
use dotenv::dotenv;
use reqwest::multipart::{Form, Part};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;

/// Get the config directory path based on the platform
fn get_config_dir() -> PathBuf {
    if let Some(config_dir) = dirs::config_dir() {
        // Standard config directories:
        // - Linux/macOS: ~/.config/alldebrid-cli/
        // - Windows: C:\Users\Username\AppData\Roaming\alldebrid-cli\
        config_dir.join("alldebrid-cli")
    } else {
        // Fallback to current directory
        PathBuf::from(".")
    }
}

/// Get the API key from the config file or environment variable
fn get_api_key() -> Result<String> {
    // First try to get from environment variable
    if let Ok(key) = env::var("ALLDEBRID_API_KEY") {
        return Ok(key);
    }
    
    // Try .env file in current directory as fallback
    dotenv().ok();
    if let Ok(key) = env::var("ALLDEBRID_API_KEY") {
        return Ok(key);
    }
    
    // If not found in environment, try to get from config file
    let config_path = get_config_dir().join("alldebrid-cli.conf");
    
    if config_path.exists() {
        let mut file = fs::File::open(&config_path)
            .context(format!("Failed to open config file at {:?}", config_path))?;
            
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Failed to read config file")?;
            
        // Parse the config file (simple key=value format)
        for line in contents.lines() {
            let line = line.trim();
            if line.starts_with("ALLDEBRID_API_KEY=") {
                let key = line.strip_prefix("ALLDEBRID_API_KEY=").unwrap_or("").trim();
                if !key.is_empty() {
                    return Ok(key.to_string());
                }
            }
        }
    }
    
    // If all else fails, return an error
    Err(anyhow::anyhow!("API key not found. Please set ALLDEBRID_API_KEY in your environment, \
                         in a .env file, or in the config file at {:?}", config_path))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from config or environment
    let api_key = get_api_key()?;
    
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