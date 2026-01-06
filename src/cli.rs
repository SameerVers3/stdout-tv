use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "stdout-tv")]
#[command(about = "Play YouTube videos as ASCII in your terminal", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Play a YouTube video as ASCII in the terminal
    Play {
        /// YouTube URL
        url: String,

        /// Terminal width in characters
        #[arg(long, default_value_t = 80)]
        width: u16,

        /// Terminal height in characters
        #[arg(long, default_value_t = 45)]
        height: u16,

        /// Frames per second
        #[arg(long, default_value_t = 24)]
        fps: u8,

        /// Custom charset (5-50 characters)
        #[arg(long, value_parser = validate_charset)]
        charset: Option<String>,

        /// Predefined charset preset
        #[arg(long, value_enum, default_value_t = CharsetPreset::Default)]
        charset_preset: CharsetPreset,

        /// Invert brightness
        #[arg(long, default_value_t = false)]
        invert: bool,

        /// Enable ANSI color
        #[arg(long, default_value_t = false)]
        color: bool,

        /// Disable audio playback
        #[arg(long, default_value_t = false)]
        no_audio: bool,

        /// Custom yt-dlp executable path
        #[arg(long, default_value = "yt-dlp")]
        yt_dlp_path: String,

        /// Custom ffmpeg executable path
        #[arg(long, default_value = "ffmpeg")]
        ffmpeg_path: String,
    },

    /// Show video metadata (title, duration, resolution, available formats)
    Info {
        /// YouTube URL
        url: String,
    },

    /// Check if all dependencies are installed
    Check,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CharsetPreset {
    /// Default character set
    Default,
    /// Dense character set for more detail
    Dense,
    /// Block characters
    Blocks,
}

fn validate_charset(s: &str) -> Result<String, String> {
    let len = s.chars().count();
    if len < 5 {
        Err(format!("charset must be at least 5 characters (got {})", len))
    } else if len > 50 {
        Err(format!("charset must be at most 50 characters (got {})", len))
    } else {
        Ok(s.to_string())
    }
}
