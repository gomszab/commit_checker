use std::rc::Rc;

use colored::Colorize;

use spinoff::{Color, Spinner, spinners};

use crate::api::{Handler, HandlerResult};

pub struct Context {
    handlers: Vec<Rc<dyn Handler>>,
    pub staged_files: Vec<String>,
    pub file_contents: Vec<String>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            handlers: Vec::new(),
            staged_files: Vec::new(),
            file_contents: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Rc<dyn Handler>) {
        self.handlers.push(handler);
    }

    pub fn run(mut self) -> Result<bool, String> {
        let mut errored = false;
        for i in 0..self.handlers.len() {
            let handler = self.handlers[i].clone();
            let mut spinner =
                Spinner::new(spinners::Circle, handler.title().to_string(), Color::Blue);

            let result = handler.handle(&mut self);
            spinner.stop();

            match result {
                HandlerResult::Ok => println!("{}", handler.success_message().green()),
                HandlerResult::SoftErrors(errors) => {
                    errored = true;
                    print_errors(errors);
                }
                HandlerResult::FatalError(error) => return Err(error),
            };
        }

        Ok(errored)
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

fn print_errors(errors: Vec<String>) {
    for error in errors {
        eprintln!("{}", error.red());
    }
}
