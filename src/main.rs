use std::io::{self, Read, stdin};

fn main() {
    for b in std::io::stdin().bytes() {
        let _char = b.unwrap() as char;
        print!("{}", _char)
    }
}
