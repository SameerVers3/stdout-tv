mod cli;
mod commands;
mod video;
mod utils;

use clap::Parser;
use cli::{Cli, Commands, CharsetPreset, resolve_dimensions};
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
            no_color,
            no_audio,
            yt_dlp_path,
            ffmpeg_path,
        } => {
            
            let (width, height) = resolve_dimensions(width, height);

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
                    color: !no_color,
                    invert,
                    charset,
                    charset_preset: lib_preset,
                    ..Default::default()
                },
                fps,
                !no_audio,
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