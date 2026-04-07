use serde_json::Value;
use std::env;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <instagram_username>", args[0]);
        std::process::exit(1);
    }

    let username = &args[1];
    let username = username.trim_end_matches('/').split('/').last().unwrap_or(username);

    println!("Fetching profile information for '{}'...", username);

    let url = format!("https://i.instagram.com/api/v1/users/web_profile_info/?username={}", username);

    let curl_output = Command::new("curl")
        .arg("-s")
        .arg("-H")
        .arg("User-Agent: Instagram 219.0.0.12.117 Android")
        .arg("-H")
        .arg("X-IG-App-ID: 936619743392459")
        .arg(&url)
        .output()?;

    if !curl_output.status.success() {
        eprintln!("Failed to run curl command.");
        std::process::exit(1);
    }

    let body = String::from_utf8_lossy(&curl_output.stdout);

    if body.contains("429 Too Many Requests") || body.trim().is_empty() {
        eprintln!("Failed to fetch profile info. Instagram returned 429 Too Many Requests or empty response.");
        std::process::exit(1);
    }

    let data: Value = match serde_json::from_str(&body) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to parse JSON response: {}", e);
            std::process::exit(1);
        }
    };

    let user_id = match data.pointer("/data/user/id").and_then(|v| v.as_str()) {
        Some(id) => id,
        None => {
            eprintln!("Could not parse user ID from the response.");
            std::process::exit(1);
        }
    };

    println!("Found Internal User ID: {}", user_id);

    let gallery_dl_url = format!("https://www.instagram.com/id:{}/", user_id);
    
    println!("Starting gallery-dl download for: {}", gallery_dl_url);
    
    let mut child = Command::new("gallery-dl")
        .arg(&gallery_dl_url)
        .spawn()?;

    let status = child.wait()?;

    match status.success() {
        true => println!("Download completed successfully!"),
        false => eprintln!("gallery-dl exited with status: {}", status),
    }

    Ok(())
}
