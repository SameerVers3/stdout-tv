use clap::{Parser, Subcommand, ValueEnum};

/// Default aspect ratio (16:9)
const ASPECT_WIDTH: f32 = 16.0;
const ASPECT_HEIGHT: f32 = 9.0;

#[derive(Parser)]
#[command(name = "stdout-tv")]
#[command(version)]
#[command(about = "Play YouTube videos as ASCII in your terminal")]
#[command(propagate_version = true)]
#[command(arg_required_else_help = true)]
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

        /// Terminal width in characters (height auto-calculated for 16:9 if not specified)
        #[arg(long)]
        width: Option<u16>,

        /// Terminal height in characters (width auto-calculated for 16:9 if not specified)
        #[arg(long)]
        height: Option<u16>,

        /// Frames per second (defaults to source video FPS)
        #[arg(long)]
        fps: Option<u8>,

        /// Custom charset (5-50 characters)
        #[arg(long, value_parser = validate_charset)]
        charset: Option<String>,

        /// Predefined charset preset
        #[arg(long, value_enum, default_value_t = CharsetPreset::Default)]
        charset_preset: CharsetPreset,

        /// Invert brightness
        #[arg(long, default_value_t = false)]
        invert: bool,

        /// Disable ANSI color
        #[arg(long, default_value_t = false)]
        no_color: bool,

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

        /// Custom yt-dlp executable path
        #[arg(long, default_value = "yt-dlp")]
        yt_dlp_path: String,
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
        Err(format!(
            "charset must be at least 5 characters (got {})",
            len
        ))
    } else if len > 50 {
        Err(format!(
            "charset must be at most 50 characters (got {})",
            len
        ))
    } else {
        Ok(s.to_string())
    }
}

/// Resolve width and height maintaining 16:9 aspect ratio
pub fn resolve_dimensions(width: Option<u16>, height: Option<u16>) -> (u16, u16) {
    if let (Some(w), Some(h)) = (width, height) {
        (w, h)
    } else if let Some(w) = width {
        let h = ((w as f32) * ASPECT_HEIGHT / ASPECT_WIDTH).round() as u16;
        (w, h)
    } else if let Some(h) = height {
        let w = ((h as f32) * ASPECT_WIDTH / ASPECT_HEIGHT).round() as u16;
        (w, h)
    } else {
        (80, 45)
    }
}
