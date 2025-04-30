use anyhow::{Context, Result};
use dotenv::dotenv;
use reqwest::multipart::{Form, Part};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::{self, Read, BufRead};

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

/// Process a single URL with the AllDebrid API
async fn process_url(client: &reqwest::Client, api_key: &str, url: &str, password: &str) -> Result<()> {
    // Prepare the form data
    let mut form = Form::new().part("link", Part::text(url.to_string()));
    
    // Add password if not empty
    if !password.is_empty() {
        form = form.part("password", Part::text(password.to_string()));
    }
    
    // Make the API request
    let response = client
        .post("http://api.alldebrid.com/v4/link/unlock")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .send()
        .await
        .context(format!("Failed to send request to AllDebrid API for URL: {}", url))?;
    
    // Parse response as JSON
    let response_json: Value = response
        .json()
        .await
        .context(format!("Failed to parse API response as JSON for URL: {}", url))?;
    
    // Check if request was successful
    if response_json["status"] == "success" {
        // Extract and print the "link" field
        if let Some(link) = response_json["data"]["link"].as_str() {
            println!("{} -> {}", url, link);
        } else {
            eprintln!("{} -> Error: Link field not found in response", url);
        }
    } else {
        // Print error message if request failed
        eprintln!("{} -> Error: {}", url, response_json);
    }
    
    Ok(())
}

/// Print usage information
fn print_usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!("  {} URL [PASSWORD]", program_name);
    eprintln!("  {} -i|--input FILENAME [PASSWORD]", program_name);
    eprintln!("");
    eprintln!("Options:");
    eprintln!("  -i, --input FILENAME    Process multiple URLs from a file (one URL per line)");
    eprintln!("  -h, --help              Display this help message");
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get API key from config or environment
    let api_key = get_api_key()?;
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
        print_usage(&args[0]);
        return Ok(());
    }
    
    // Create HTTP client - reuse for multiple requests
    let client = reqwest::Client::new();
    
    // Process input based on arguments
    if args[1] == "-i" || args[1] == "--input" {
        // Process multiple URLs from a file
        if args.len() < 3 {
            eprintln!("Error: Input file not specified");
            print_usage(&args[0]);
            return Ok(());
        }
        
        let input_file = &args[2];
        let password = if args.len() > 3 { &args[3] } else { "" };
        
        // Open and read the file
        let file = fs::File::open(input_file)
            .context(format!("Failed to open input file: {}", input_file))?;
        
        let reader = io::BufReader::new(file);
        
        // Process each line as a URL
        for line in reader.lines() {
            let url = line.context("Failed to read line from input file")?;
            let trimmed_url = url.trim();
            
            // Skip empty lines and comment lines
            if !trimmed_url.is_empty() && !trimmed_url.starts_with('#') {
                if let Err(e) = process_url(&client, &api_key, trimmed_url, password).await {
                    eprintln!("{} -> Error: {}", trimmed_url, e);
                }
            }
        }
    } else {
        // Process a single URL
        let url = &args[1];
        let password = if args.len() > 2 { &args[2] } else { "" };
        
        process_url(&client, &api_key, url, password).await?;
    }
    
    Ok(())
}
