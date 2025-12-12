use std::rc::Rc;

use colored::Colorize;
use spinoff::{Color, Spinner, spinners};

use crate::api::Handler;

pub struct Context {
    handlers: Vec<Rc<dyn Handler>>,
    current: usize,
    pub staged_files: Vec<String>,
    pub file_contents: Vec<String>,
    pub errors: Vec<String>,
    pub spinner: Option<Spinner>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            handlers: Vec::new(),
            current: 0,
            staged_files: Vec::new(),
            file_contents: Vec::new(),
            errors: Vec::new(),
            spinner: None,
        }
    }

    pub fn register_handler(&mut self, handler: Rc<dyn Handler>) {
        self.handlers.push(handler);
    }

    pub fn run(mut self) -> Result<Vec<String>, String> {
        self.next()?;

        Ok(self.errors)
    }

    pub fn next(&mut self) -> Result<(), String> {
        if self.current >= self.handlers.len() {
            return Ok(());
        }

        let handler = self.handlers[self.current].clone();

        self.spinner = Some(Spinner::new(
            spinners::Circle,
            handler.clone().title().to_string(),
            Color::Blue,
        ));
        let result = handler.handle(self);
        result
    }

    pub fn end_of_handle(&mut self, message: Option<&str>) -> Result<(), String> {
        if let Some(spinner) = &mut self.spinner {
            spinner.stop();
            let handler = self.handlers[self.current].clone();
            if message.is_none() {
                println!("{}", handler.success_message().green());
                self.current += 1;
                self.next()
            } else {
                Err(format!("{}", message.unwrap()))
            }
        } else {
            Err(format!("Nem vart hiba, goto gomszab"))
        }
    }
}

impl Context {
    pub fn add_staged_files(&mut self, filename: &str) {
        self.staged_files.push(filename.to_string());
    }

    pub fn add_file_contents(&mut self, file_content: &str) {
        self.file_contents.push(file_content.to_string());
    }
}
