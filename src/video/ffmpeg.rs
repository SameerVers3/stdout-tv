use std::process::{Child, Command, Stdio};
use image::RgbImage;
use crate::utils;

pub fn spawn_ffmpeg(ffmpeg_path: &str, video_url: &str, width: u16, height: u16, fps: f32, audio_enabled: bool) -> Result<Child, Box<dyn std::error::Error>> {

    let ffmpeg = if ffmpeg_path.is_empty() { "ffmpeg" } else { ffmpeg_path };

    let scale_filter = format!("scale={}:{}", width, height);
    let fps_str = format!("{}", fps);

    let child: Child;

    if audio_enabled {
        let (audio_format, audio_device) = utils::get_audio_output();

        child = Command::new(ffmpeg)
            .args([
                "-fflags", "nobuffer", 
                "-flags", "low_delay",
                "-i", video_url,
                "-map", "0:v",
                "-r", &fps_str,
                "-vf", &scale_filter,
                "-pix_fmt", "rgb24",
                "-f", "rawvideo",
                "pipe:1",
                // audio 
                "-map", "0:a",
                "-f", audio_format,
                "-ac", "2",
                "-ar", "44100",
                audio_device,
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())  
            .spawn()?;
    } else {

        // Video only
        child = Command::new(ffmpeg)
            .args([
                "-i", video_url,
                "-r", &fps_str,
                "-vf", &scale_filter,
                "-pix_fmt", "rgb24",
                "-f", "rawvideo",
                "-"
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;
    };

    Ok(child)
}


pub fn buffer_to_image(buffer: &[u8], width: u32, height: u32) -> RgbImage {
    use image::Rgb;
    let mut img = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let i = ((y * width + x) * 3) as usize;
            img.put_pixel(x, y, Rgb([buffer[i], buffer[i + 1], buffer[i + 2]]));
        }
    }
    img
}
