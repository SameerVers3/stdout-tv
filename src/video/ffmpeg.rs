use std::process::{Child, Command, Stdio};
use std::io;
use image::RgbImage;


pub fn spawn_ffmpeg(ffmpeg_path: &str, video_url: &str, width: u16, height: u16) -> Result<Child, Box<dyn std::error::Error>> {

    let ffmpeg = if ffmpeg_path.is_empty() { "ffmpeg" } else { ffmpeg_path };

    // 3 bytes for rgb * height * width = number of bytes per frame
    let _frame_size = (width * height * 3) as usize;

    
    let mut child = Command::new("ffmpeg")
        .args([
            "-i", &video_url,
            "-vf", &format!("scale={}:{}", width, height),
            "-pix_fmt", "rgb24",
            "-f", "rawvideo",
            "-"
        ])
        .stdout(Stdio::piped())
        .spawn()?;

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
