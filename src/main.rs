mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Play {
            url: _url,
            width: _width,
            height: _height,
            fps: _fps,
            charset: _charset,
            charset_preset: _charset_preset,
            invert: _invert,
            color: _color,
            no_audio: _no_audio,
            yt_dlp_path: _yt_dlp_path,
            ffmpeg_path: _ffmpeg_path,
        } => {
            // TODO: Implement play command
            // println!("Playing: {}", _url);
            // println!("Resolution: {}x{}", _width, _height);
            // println!("FPS: {}", _fps);
            // println!("Charset: {:?}", _charset);
            // println!("Charset Preset: {:?}", _charset_preset);
            // println!("Invert: {}", _invert);
            // println!("Color: {}", _color);
            // println!("No Audio: {}", _no_audio);
            // println!("yt-dlp path: {}", _yt_dlp_path);
            // println!("ffmpeg path: {}", _ffmpeg_path);
        }
        Commands::Info { url: _url } => {
            // TODO: Implement info command
            // println!("Fetching info for: {}", _url);
        }
        Commands::Check => {
            // TODO: Implement check command
            // - Check if yt-dlp exists
            // - Check if ffmpeg exists
            // - Check if terminal supports ANSI
            commands::check::run();
        }
    }
}