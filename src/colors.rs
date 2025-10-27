pub const RESET: &str = "\x1b[0m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const BLUE: &str = "\x1b[34m";
pub const PURPLE: &str = "\x1b[35m";

use std::io::{self, Write};

pub fn colored_text(text: &str, color: &str) -> String {
    format!("{}{}{}", color, text, RESET)
}

pub fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}
