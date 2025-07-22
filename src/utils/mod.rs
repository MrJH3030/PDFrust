pub mod strings;

use std::io::{self, Write};

pub fn clear_screen_line(){
    print!("\x1B[1A\x1B[2K");
    io::stdout().flush().unwrap();
}