mod cli;
mod commands;
mod video;
mod utils;

use clap::Parser;
use cli::{Cli, Commands, CharsetPreset};
use pixel2ascii::{AsciiOptions, convert::CharsetPreset as LibCharsetPreset};


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Play {
            url: video_url,
            width,
            height,
            fps,
            charset,
            charset_preset,
            invert,
            color,
            no_audio: _no_audio,
            yt_dlp_path,
            ffmpeg_path,
        } => {
            
            let lib_preset = match charset_preset {
                CharsetPreset::Default => LibCharsetPreset::Default,
                CharsetPreset::Dense => LibCharsetPreset::Dense,
                CharsetPreset::Blocks => LibCharsetPreset::Blocks,
            };

            commands::play::run(
                &video_url,
                &yt_dlp_path, 
                &ffmpeg_path,
                width, 
                height, 
                AsciiOptions {
                    width: width as u32,
                    color,
                    invert,
                    charset,
                    charset_preset: lib_preset,
                    ..Default::default()
                },
                fps
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