#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
use std::io::{self, stdout, Write};
use std::{env, fs};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

struct Terminal {
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn write_raw(&self, data: &String) {
        println!("{}", data)
    }

    pub fn init() -> Self {
        return Terminal {
            _stdout: stdout().into_raw_mode().unwrap(),
        };
    }

    pub fn _flush(&self) -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
}

struct Document {
    pub content: String,
    pub _name: String,
}

impl Document {
    pub fn init(file_name: &String) -> Self {
        let result = fs::read_to_string(file_name);
        let content = result.unwrap();
        return Self {
            _name: file_name.clone(),
            content: content,
        };
    }
}

struct Application {
    terminal: Terminal,
}

impl Application {
    pub fn init() -> Self {
        Self {
            terminal: Terminal::init(),
        }
    }

    pub fn run(&self, document: &Document) {
        loop {
            let key = Terminal::read_key().unwrap();
            match key {
                Key::Char('\n') => self.terminal.write_raw(&document.content),
                Key::Esc => break,
                _ => (),
            }
        }
    }
}

fn main() {
    let app = Application::init();
    let args: Vec<String> = env::args().collect();
    let maybe_file_name = args.get(1);
    if maybe_file_name.is_none() {
        return;
    }

    let file_name = maybe_file_name.unwrap();
    let document = Document::init(file_name);
    app.run(&document);
}
