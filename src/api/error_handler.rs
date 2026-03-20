use colored::Colorize;
use rust_i18n::t;
use crate::api::file_context::{FileFeedback};

pub struct ErrorHandler {
    pub errored_files: Vec<FileFeedback>,
    pub ok_files: Vec<FileFeedback>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        let err_handler = ErrorHandler {
            errored_files: Vec::new(),
            ok_files: Vec::new(),
        };
        err_handler
    }

    pub fn add_result(&mut self, result: Result<FileFeedback, String>) {
        match result {
            Ok(file_feedback) => {
                if file_feedback.tasks.iter().any(|task| task.errored) {
                    self.errored_files.push(file_feedback);
                } else {
                    self.ok_files.push(file_feedback);
                }
            }
            Err(_err) => {
                ErrorHandler::print_error(t!("SW03").to_string());
            }
        }
    }

    pub fn is_errored(&self) -> bool {
        self.errored_files.len() != 0
    }

    pub fn print_errors(errors: &Vec<String>) {
        for error in errors {
            eprintln!("{}", error.red());
        }
    }

    pub fn print_oks(oks: &Vec<String>) {
        for ok in oks {
            println!("{}", ok.green());
        }
    }

    pub fn print_error(message: String) {
        eprintln!("{}", message.red());
    }

    pub fn print_ok(message: String) {
        println!("{}", message.green());
    }
}