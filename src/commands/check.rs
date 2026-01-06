use std::process::Command;

pub fn run() {
    println!("Checking dependencies for stdout-tv...\n");

    let yt_dlp_ok = check_command("yt-dlp", "--version");
    let ffmpeg_ok = check_command("ffmpeg", "-version");

    if yt_dlp_ok && ffmpeg_ok {
        println!("\nAll dependencies are installed!");
    } else {
        println!("\nSome dependencies are missing. Please install them.");
        if !yt_dlp_ok {
            println!("- yt-dlp: Install with `pacman -S yt-dlp` or your package manager");
        }
        if !ffmpeg_ok {
            println!("- ffmpeg: Install with `pacman -S ffmpeg` or your package manager");
        }
    }
}

fn check_command(cmd: &str, version_flag: &str) -> bool {
    match Command::new(cmd).arg(version_flag).output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("{} found: {}", cmd, version.lines().next().unwrap_or(""));
            true
        }
        _ => {
            println!("{} not found", cmd);
            false
        }
    }
}