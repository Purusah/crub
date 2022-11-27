#![allow(dead_code)]
use std::{env, fs};

struct Output {}

impl Output {
    pub fn write_raw(&self, data: &String) {
        println!("{}", data)
    }

    pub fn init() -> Self {
        return Output {};
    }
}

struct Document {
    pub content: String,
    pub name: String,
}

impl Document {
    pub fn init(file_name: &String) -> Self {
        let result = fs::read_to_string(file_name);
        let content = result.unwrap();
        return Self {
            name: file_name.clone(),
            content: content,
        };
    }
}

struct Application {
    output: Output,
}

impl Application {
    pub fn init() -> Self {
        Self {
            output: Output::init(),
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
    app.output.write_raw(&document.content);
}
