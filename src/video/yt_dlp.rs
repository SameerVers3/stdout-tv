use std::process::Command;

pub fn get_video_url(youtube_url: &str, yt_dlp_path: &str) -> Result<String, Box<dyn std::error::Error>> {

    let yt_dlp = if yt_dlp_path.is_empty() { "yt-dlp" } else { yt_dlp_path };

    let output = Command::new(yt_dlp)
        .args(["-f", "bestvideo", "-g", youtube_url])
        .output()?;
    
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}