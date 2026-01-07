pub fn hide_cursor() { 
    print!("\x1B[?25l"); 
}

pub fn show_cursor() { 
    print!("\x1B[?25h"); 
}

pub fn clear_screen() {
    print!("\x1B[2J"); 
}

pub fn move_cursor_home() {
    print!("\x1B[H"); 
}

pub fn get_audio_output() -> (&'static str, &'static str) {
    #[cfg(target_os = "linux")]
    { ("pulse", "default") }
    
    #[cfg(target_os = "macos")]
    { ("audiotoolbox", "default") }
    
    #[cfg(target_os = "windows")]
    { ("dsound", "default") }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    { ("sdl", "default") }
}

pub struct CursorGuard;

impl CursorGuard {
    pub fn new() -> Self {
        hide_cursor();
        clear_screen();
        Self
    }
}

impl Drop for CursorGuard {
    fn drop(&mut self) {
        show_cursor();
    }
}

