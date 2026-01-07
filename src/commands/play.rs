use std::process::{ChildStdout, Command};
use std::io::Read;
use crate::video::{yt_dlp, ffmpeg};
use pixel2ascii::{image_to_ascii, AsciiOptions};
use std::time::{Instant, Duration};

pub fn run(youtube_url: &str, yt_dlp_path: &str, _ffmpeg_path: &str, width: u16, height: u16, options: AsciiOptions, fps: u8) {
    
    // get the URL
    let url = yt_dlp::get_video_url(youtube_url, yt_dlp_path).unwrap();
    
    println!("finally got the video hehe:  {}", url);

    // Spawn ffmpeg as a continuous video stream

    //=============================================

    let mut child = ffmpeg::spawn_ffmpeg(_ffmpeg_path, &url, width, height).unwrap();


    let mut stdout: ChildStdout = child.stdout.take().unwrap();

    // frame size
    let frame_size = (width * height * 3) as usize;

    let mut buffer = vec![0u8; frame_size];

    // Clear screen and hide cursor {move it to utils later}

    print!("\x1B[2J\x1B[?25l"); 
    let frame_duration = Duration::from_secs_f64(1.0 / fps as f64);

    println!("Now trying to play the thing");


    loop {
        println!("loop started......");

        let start = Instant::now();

        // Read one frame
        if stdout.read_exact(&mut buffer).is_err() {
            println!("Errorrrr.........");
            break; // EOF or error → stop
        }

        // Convert to image
        let img = ffmpeg::buffer_to_image(&buffer, width as u32, height as u32);

        // Convert to ASCII
        let ascii_frame = image_to_ascii(&img, &options);

        // Print frame (move cursor to top-left)
        print!("\x1B[H");
        println!("{}", ascii_frame);

        // FPS control
        let elapsed = start.elapsed();

        if elapsed < frame_duration {
            std::thread::sleep(frame_duration - elapsed);
        }

        
    }

    print!("\x1B[?25h");
}