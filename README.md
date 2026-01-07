<p align="center">
  <img src="https://raw.githubusercontent.com/SameerVers3/stdout-tv/main/assets/logo.png" alt="stdout-tv logo" width="200"/>
</p>

<h1 align="center">📺 stdout-tv</h1>

<p align="center">
  <strong>🎬 Transform YouTube into ASCII cinema - right in your terminal</strong>
</p>

<p align="center">
  <em>Because who needs 4K when you have 80 columns.</em>
</p>

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-demo">Demo</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-usage">Usage</a> •
  <a href="#%EF%B8%8F-options">Options</a> •
  <a href="#-dependencies">Dependencies</a>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.75+-93450a.svg?style=flat-square&logo=rust" alt="Rust"/></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License"/></a>
  <a href="https://github.com/SameerVers3/stdout-tv/stargazers"><img src="https://img.shields.io/github/stars/SameerVers3/stdout-tv?style=flat-square&logo=github&color=yellow" alt="Stars"/></a>
</p>

<br/>

---

## Features

-  **Stream YouTube videos** as ASCII art in real-time
-  **Full color support** with ANSI colors
-  **Synchronized audio** playback
-  **Multiple charset presets** for different styles

---

##  Demo

https://github.com/user-attachments/assets/d132860c-de12-4f78-ab4a-c96425d2285a


```bash
# One command. Instant ASCII cinema.

stdout-tv play "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```

<div align="center">

  <!-- Top row: two images -->
  <img src="https://github.com/user-attachments/assets/a27f77e0-4e15-43c7-8668-3c8ba8b22cfa" alt="master_po" width="400" >
  <img src="https://github.com/user-attachments/assets/454e36f3-1336-4790-9831-34f7483864d7" alt="dense" width="400">
  
  <br><br> <!-- space between rows -->

  <!-- Bottom row: single image -->
  <img src="https://github.com/user-attachments/assets/dc7a2193-573e-40f4-b842-1b9196bdb697" alt="monochrome" width="450" style="margin:10px;">

</div>

---

##  Installation

### Prerequisites

> **Note:** Make sure you have [Rust](https://rustup.rs/) installed (1.75+)

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/SameerVers3/stdout-tv.git
cd stdout-tv

# Build with Cargo (optimized release build)
cargo build --release

# Run it!
./target/release/stdout-tv --help
```

### Quick Install Script

```bash
# One-liner install
curl -sSL https://raw.githubusercontent.com/SameerVers3/stdout-tv/main/install.sh | bash
```

### Add to PATH

```bash
# Linux/macOS - Add to your local bin
sudo cp ./target/release/stdout-tv /usr/local/bin/

# Or symlink it
ln -s $(pwd)/target/release/stdout-tv ~/.local/bin/stdout-tv
```

---

##  Usage

###  Play a Video

```bash
# Basic - just paste the URL
stdout-tv play "https://www.youtube.com/watch?v=VIDEO_ID"

#  Custom size (height auto-calculated for 16:9)
stdout-tv play "<url>" --width 120

#  Experiment with charsets
stdout-tv play "<url>" --charset-preset dense    # More detail
stdout-tv play "<url>" --charset-preset blocks   # Pixel art vibes

#  Silent mode (no audio)
stdout-tv play "<url>" --no-audio

#  Invert colors (for light terminals)
stdout-tv play "<url>" --invert

#  Custom FPS (default: source video FPS)
stdout-tv play "<url>" --fps 30
```

###  Get Video Info

```bash
stdout-tv info "https://www.youtube.com/watch?v=VIDEO_ID"
```

<details>
<summary> Example Output</summary>

```
Title: Never Gonna Give You Up
Uploader: Rick Astley
Upload Date: 20091025
Duration: 3m 32s
View Count: 1500000000
Like Count: 15000000

Description:
The official video for "Never Gonna Give You Up" by Rick Astley...

Tags:
rick astley | never gonna give you up | rickroll | 80s |
```
</details>

###  Check Dependencies

```bash
stdout-tv check
```

```
Checking dependencies for stdout-tv...

✓ yt-dlp found: yt-dlp 2024.12.01
✓ ffmpeg found: ffmpeg version 6.1.1

All dependencies are installed!
```

---

##  Options

<details open>
<summary><b>Play Command Options</b></summary>

| Option | Description | Default |
|:-------|:------------|:--------|
| `--width <WIDTH>` | Terminal width in characters | `80` |
| `--height <HEIGHT>` | Terminal height in characters | Auto (16:9) |
| `--fps <FPS>` | Frames per second | Source FPS |
| `--charset <CHARSET>` | Custom charset (5-50 chars) | — |
| `--charset-preset <PRESET>` | Preset: `default` \| `dense` \| `blocks` | `default` |
| `--invert` | Invert brightness mapping | `false` |
| `--color` | Enable ANSI color output | `true` |
| `--no-audio` | Disable audio playback | `false` |
| `--yt-dlp-path <PATH>` | Custom yt-dlp executable | `yt-dlp` |
| `--ffmpeg-path <PATH>` | Custom ffmpeg executable | `ffmpeg` |

</details>

###  Charset Presets

| Preset | Style | Best For |
|:-------|:------|:---------|
| `default` | `@%#*+=-:. ` | General purpose, balanced |
| `dense` | Extended ASCII range | High detail, complex scenes |
| `blocks` | `█▓▒░ ` | Retro pixel art look |

---

## 🔧 Dependencies

stdout-tv requires these external tools to work its magic:

| Tool | Purpose | Install |
|:-----|:--------|:--------|
| <img src="https://img.shields.io/badge/yt--dlp-FF0000?style=flat-square&logo=youtube&logoColor=white" alt="yt-dlp"/> | Fetches video streams | `brew install yt-dlp` / `pacman -S yt-dlp` |
| <img src="https://img.shields.io/badge/ffmpeg-007808?style=flat-square&logo=ffmpeg&logoColor=white" alt="ffmpeg"/> | Video processing & audio | `brew install ffmpeg` / `pacman -S ffmpeg` |

>  **Tip:** Run `stdout-tv check` to verify everything is set up correctly!

---

##  Project Structure

```
stdout-tv/
├── Cargo.toml            # Project manifest
├── src/
│   ├── main.rs           # Entry point
│   ├── cli.rs            # CLI with clap
│   ├── utils.rs          # Terminal magic
│   ├── commands/
│   │   ├── play.rs       # The main show
│   │   ├── info.rs       # Video metadata
│   │   └── check.rs       # Dependency checker
│   └── video/
│       ├── yt_dlp.rs     # yt-dlp wrapper
│       └── ffmpeg.rs     # ffmpeg pipeline
```

---

## Contributing

Contributions, issues, and feature requests are welcome!

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## Implementation Notes

- This project uses **[pixel2ascii](https://crates.io/crates/pixel2ascii)**, a companion crate developed to handle image-to-ASCII conversion efficiently.
---

<p align="center">
  <b>Made with ❤️ and 🦀 Rust</b>
  <br/>
  <sub>Star ⭐ this repo if you found it useful!</sub>
</p>
