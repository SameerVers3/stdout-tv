use std::process::ChildStdout;
use std::io::Read;
use crate::video::{yt_dlp, ffmpeg};
use pixel2ascii::{image_to_ascii, AsciiOptions};
use crate::utils::{self, CursorGuard};

pub fn run(youtube_url: &str, yt_dlp_path: &str, ffmpeg_path: &str, width: u16, height: u16, options: AsciiOptions, fps: Option<u8>, audio_enabled: bool) {
    
    let _guard = CursorGuard::new();

    let actual_fps = match fps {
        Some(f) => f as f32,
        None => yt_dlp::get_video_fps(youtube_url, yt_dlp_path).unwrap_or(24.0),
    };

    // get the URL (with audio if enabled)
    let url = yt_dlp::get_video_url(youtube_url, yt_dlp_path, audio_enabled).unwrap();
    
    // Spawn ffmpeg as a continuous video stream

    let mut child = ffmpeg::spawn_ffmpeg(ffmpeg_path, &url, width, height, actual_fps, audio_enabled).unwrap();

    let mut stdout: ChildStdout = child.stdout.take().unwrap();

    // frame size
    let frame_size = (width * height * 3) as usize;

    let mut buffer = vec![0u8; frame_size];

    loop {
        // Read one frame (blocks until ffmpeg outputs next frame)
        if stdout.read_exact(&mut buffer).is_err() {
            break;
        }

        // Convert to image
        let img = ffmpeg::buffer_to_image(&buffer, width as u32, height as u32);

        // Convert to ASCII
        let ascii_frame = image_to_ascii(&img, &options);

        // Print frame (move cursor to top-left)
        utils::move_cursor_home();

        println!("{}", ascii_frame);
    }
}