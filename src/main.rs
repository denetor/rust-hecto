use std::io::{self, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        // exit when encounters a 'q' key
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }
}
