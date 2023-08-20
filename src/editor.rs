use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die(e: std::io::Error) {
    panic!("{:?}", e);
}

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in io::stdin().keys() {
            match key {
                Ok(key) => match key {
                    Key::Char(c) => {
                        println!("{}\r", c);
                    }
                    Key::Ctrl('q') => break,
                    Key::Esc => break,
                    _ => println!("{:?}\r", key),
                },
                Err(err) => die(err),
            }
        }
    }
}
