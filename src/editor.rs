use std::io::{self, stdout, Error};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn die(e: &Error) {
    panic!("{e:?}");
}

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = self.read_key()?;
        match pressed_key {
            Key::Ctrl('q') | Key::Esc => panic!("Program End"),
            _ => (),
        };
        Ok(())
    }

    fn read_key(&self) -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
    
}

