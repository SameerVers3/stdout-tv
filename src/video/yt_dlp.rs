use std::process::Command;

pub fn get_video_url(youtube_url: &str, yt_dlp_path: &str, with_audio: bool) -> Result<String, Box<dyn std::error::Error>> {

    let yt_dlp = if yt_dlp_path.is_empty() { "yt-dlp" } else { yt_dlp_path };

    let format = if with_audio { "best" } else { "bestvideo" };

    let output = Command::new(yt_dlp)
        .args(["-f", format, "-g", youtube_url])
        .output()?;
    
    Ok(String::from_utf8(output.stdout)?.trim().to_string())
}

/// Get the FPS of the video
pub fn get_video_fps(youtube_url: &str, yt_dlp_path: &str) -> Result<f32, Box<dyn std::error::Error>> {
    let yt_dlp = if yt_dlp_path.is_empty() { "yt-dlp" } else { yt_dlp_path };

    let output = Command::new(yt_dlp)
        .args(["--print", "%(fps)s", youtube_url])
        .output()?;

    let fps_str = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(fps_str.parse::<f32>().unwrap_or(24.0))
}