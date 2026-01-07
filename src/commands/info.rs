use std::process::Command;
use serde_json::Value;

pub fn run(url: &str, yt_dlp_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    
    let yt_dlp = if yt_dlp_path.is_empty() { "yt-dlp" } else { yt_dlp_path };

    let output = Command::new(yt_dlp)
        .args(["-j", url])
        .output()?;

    if !output.status.success() {
        eprintln!("Failed to fetch video info using {}", yt_dlp);
        return Ok(());
    }

    let mut json: Value = serde_json::from_slice(&output.stdout)?;

    json.as_object_mut().map(|obj| obj.remove("formats"));

    println!("Title: {}", json["title"].as_str().unwrap_or("N/A"));
    println!("Uploader: {}", json["uploader"].as_str().unwrap_or("N/A"));
    println!("Upload Date: {}", json["upload_date"].as_str().unwrap_or("N/A"));

    if let Some(duration) = json["duration"].as_i64() {
        let mins = duration / 60;
        let secs = duration % 60;
        println!("Duration: {}m {}s", mins, secs);
    } else {
        println!("Duration: N/A");
    }

    println!("View Count: {}", json["view_count"].as_i64().unwrap_or(0));
    println!("Like Count: {}", json["like_count"].as_i64().unwrap_or(0));

    println!("\nDescription:\n{}", json["description"].as_str().unwrap_or("N/A"));

    println!("\nTags:");
    if let Some(tags) = json["tags"].as_array() {
        if tags.is_empty() {
            println!("N/A");
        } else {
            for tag in tags {
                if let Some(tag_str) = tag.as_str() {
                    print!("{} | ", tag_str);
                }
            }
            println!();
        }
    } else {
        println!("N/A");
    }

    Ok(())
}
