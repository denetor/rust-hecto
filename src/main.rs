use std::io::{self, Read};

fn main() {
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;
        println!("{}", c);
        // exit when encounters a 'q' key
        if c == 'q' {
            break;
        }
    }
}
