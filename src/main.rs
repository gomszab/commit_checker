mod api;
mod rules;
use spinoff::{Color, Spinner, spinners};
use std::process::{Command, exit};
use std::rc::Rc;

use colored::Colorize;
use oxc::allocator::Allocator;
use rust_i18n::t;
use sys_language::detect_system_language;

use crate::api::FileContext;
use crate::api::error_handler::ErrorHandler;
use crate::rules::{
    ClassChecker, ClassNameChecker, CommentChecker, FunctionJsDocChecker, FunctionNameChecker,
    JsDocTypeChecker, PropertyJsDocChecker, PropertyNameChecker, TypeJsDocChecker,
    TypedefJsDocChecker, UnusedVariableChecker, VarKeywordChecker, VariableJsDocChecker,
    VariableNameChecker,
};

// Setting up i18n!
// This have to be in main, otherwise it won't work!
rust_i18n::i18n!("i18n", fallback = "en");

fn main() {
    // Reading and setting language
    let lang = detect_system_language();
    rust_i18n::set_locale(lang.as_str());

    // Getting staged files
    let files = match get_staged_files() {
        Ok(files) => files,
        Err(message) => {
            ErrorHandler::print_error(message);
            exit(1);
        }
    };

    // ErrorHandler
    let mut error_handler = ErrorHandler::new();

    // Needed for oxc.
    let mut allocator = Allocator::new();

    let spin_message = t!("SW01").to_string() + "\n";
    let mut spinner = Spinner::new(spinners::Circle, spin_message, Color::Blue);
    for file_name in files {
        // We do not check files other than .js files.
        if !file_name.ends_with(".js") {
            continue;
        }
        let content = match std::fs::read_to_string(&file_name) {
            Ok(content) => content,
            Err(_) => {
                let message = t!(
                    "GIT01", file_name = file_name
                ).to_string();
                ErrorHandler::print_error(message);
                exit(1);
            }
        };
        let mut context = match FileContext::new(file_name.clone(), &content, &allocator) {
            Ok(context) => context,
            Err(message) => {
                ErrorHandler::print_error(message);
                exit(1);
            }
        };

        context.register_handler(Rc::new(CommentChecker));
        context.register_handler(Rc::new(VariableJsDocChecker));
        context.register_handler(Rc::new(TypedefJsDocChecker));
        context.register_handler(Rc::new(TypeJsDocChecker));
        context.register_handler(Rc::new(JsDocTypeChecker));
        context.register_handler(Rc::new(VarKeywordChecker));

        context.register_handler(Rc::new(VariableNameChecker));
        context.register_handler(Rc::new(FunctionNameChecker));
        context.register_handler(Rc::new(FunctionJsDocChecker));
        context.register_handler(Rc::new(UnusedVariableChecker));
        // TODO Handle multiple files in case of unused functionchecker
        // context.register_handler(Rc::new(UnusedFunctionChecker));
        context.register_handler(Rc::new(ClassNameChecker));
        context.register_handler(Rc::new(ClassChecker));
        context.register_handler(Rc::new(PropertyJsDocChecker));
        context.register_handler(Rc::new(PropertyNameChecker));

        let result = context.run();
        allocator.reset();

        error_handler.add_result(result);
    }
    spinner.stop();

    show_feedback(&error_handler);
    println!();
    summarize(&error_handler);

    if error_handler.is_errored() {
        exit(1);
    }
}

fn show_feedback(error_handler: &ErrorHandler){
    // Print errored files
    for file in &error_handler.errored_files {
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

    // Print ok files
    for file in &error_handler.ok_files {
        println!("{}", t!("SW02", file_name = file.file_name).to_string());
        for task in &file.tasks {
            println!("{}", task.task_name);
            ErrorHandler::print_oks(&task.messages);
        }
        println!();
    }
}

fn summarize(error_handler: &ErrorHandler){
    for file in &error_handler.errored_files {
        let message = t!("ERR", file_name = file.file_name).to_string();
        ErrorHandler::print_error(message);
    }
    for file in &error_handler.ok_files {
        let message = t!("OK", file_name = file.file_name).to_string();
        ErrorHandler::print_ok(message);
    }
}

fn get_staged_files() -> Result<Vec<String>, String> {
    let mut staged_files = Vec::new();
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .output();
    if let Ok(content) = output {
        let files = String::from_utf8_lossy(&content.stdout);

        for filename in files.lines() {
            staged_files.push(filename.to_string());
            let diff_output = Command::new("git")
                .args(["diff", "--name-only", filename])
                .output();
            match diff_output {
                Ok(diff_content) => {
                    if !diff_content.stdout.is_empty() {
                        return Err(t!(
                            "GIT02", file_name = filename
                        ).to_string());
                    }
                }
                Err(_) => {
                    return Err(t!("GIT03").to_string());
                }
            }
        }
    } else {
        return Err(t!("GIT04").to_string());
    }

    Ok(staged_files)
}
