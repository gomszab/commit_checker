use colored::Colorize;
use crate::api::file_context::{FileFeedback};
use rust_i18n::t;

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

impl ErrorHandler {
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
                ErrorHandler::print_error("Failed to handle files".to_string());
            }
        }
    }

    pub fn show_feedback(&self) {
        // Print Errored files
        for file in &self.errored_files {
            println!("{}:", file.file_name);
            for task in &file.tasks {
                println!("{}", task.task_name);
                if task.errored {
                    ErrorHandler::print_errors(&task.messages); // print errored tasks
                } else {
                    ErrorHandler::print_oks(&task.messages); // print ok tasks
                }
            }
            println!();
        }

        // Print OK files
        for file in &self.ok_files {
            println!("{}: ✔ Minden teszt lefutott sikeresen (:", file.file_name);
            for task in &file.tasks {
                println!("{}", task.task_name);
                ErrorHandler::print_oks(&task.messages);
            }
            println!();
        }
    }

    // Summarize the file feedbacks
    pub fn summa(&self) {
        for file in &self.errored_files {
            let message = t!("ERR", file_name = file.file_name).to_string();
            ErrorHandler::print_error(message);
        }
        for file in &self.ok_files {
            let message = t!("OK", file_name = file.file_name).to_string();
            ErrorHandler::print_ok(message);
        }
    }

    pub fn is_errored(&self) -> bool {
        self.errored_files.len() != 0
    }
}