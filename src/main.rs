mod cli;
mod commands;
mod video;

use clap::Parser;
use cli::{Cli, Commands};
use pixel2ascii::{AsciiOptions};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Play {
            url: video_url,
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

            commands::play::run(
                &video_url,
                &_yt_dlp_path, 
                &_ffmpeg_path,
                _width, 
                _height, 
                AsciiOptions {
                    width: _width as u32,
                    color: true,
                    charset: _charset,
                    ..Default::default()
                },
                _fps
            );
            
        }
        Commands::Info { 
            url: _url,
            yt_dlp_path: _yt_dlp_path 
        } => {
            if let Err(e) = commands::info::run(&_url, &_yt_dlp_path) {
                eprintln!("Error: {}", e);
            }
        }
        Commands::Check => {
            commands::check::run();
        }
    }
}