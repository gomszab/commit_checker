mod api;
mod rules;

use std::process::exit;
use std::rc::Rc;

use colored::Colorize;

use crate::api::Context;
use crate::rules::{
    CommentChecker, FileContentRead, IndexJsChecker, JsDocChecker, JsDocCounter, StageHandler,
    VariableNameChecker,
};

fn main() {
    let mut context = Context::new();
    context.register_handler(Rc::new(StageHandler));
    context.register_handler(Rc::new(IndexJsChecker));
    context.register_handler(Rc::new(FileContentRead));
    context.register_handler(Rc::new(CommentChecker));
    context.register_handler(Rc::new(JsDocCounter));
    context.register_handler(Rc::new(JsDocChecker));
    context.register_handler(Rc::new(VariableNameChecker));
    let result = context.run();
    match result {
        Err(message) => {
            eprintln!("{}", message.red());
            exit(1);
        }
        Ok(errors) => {
            if errors.is_empty() {
                let message = format!("{}", "✔ Minden teszt lefutott sikeresen (:");
                println!("{}", message.green());
            } else {
                print_errors(errors);
            }
        }
    };
}

fn print_errors(errors: Vec<String>) {
    for error in errors {
        eprintln!("{}", error.red());
    }
}
