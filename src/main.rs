mod api;
mod rules;

use std::process::{Command, exit};
use std::rc::Rc;

use colored::Colorize;
use oxc::allocator::{Allocator};

use crate::api::error_handler::{ErrorHandler};
use crate::api::FileContext;
use crate::rules::{
    CommentChecker, FunctionJsDocChecker, FunctionNameChecker, JsDocTypeChecker,
    TypedefJsDocChecker, UnusedVariableChecker, VarKeywordChecker,
    VariableJsDocChecker, VariableNameChecker,
};

fn main() {
    let files = match get_staged_files() {
        Ok(files) => files,
        Err(message) => {
            ErrorHandler::print_error(message);
            exit(1);
        }
    };

    // Needed for oxc.
    let mut allocator = Allocator::new();
    let mut file_errored_flag = false;
    let mut files_errored = Vec::new();

    // Changed it to borrow, so we won't move the files, we need it to access it later...
    for file_name in files {
        // We do not check files other than .js files.
        if !file_name.ends_with(".js") {
            continue;
        }

        let content = match std::fs::read_to_string(&file_name) {
            Ok(content) => content,
            Err(_) => {
                let message = format!(
                    "nem sikerült a {file_name} fájl olvasása\nelképzelhető hogy stagelve van egy fájl, amit kitöröltél; nézd meg a git status-t, és ha zölddel ott van egy fájl, ami törölve van, futtasd a git rm --cached {file_name} parancsot)"
                );
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
        context.register_handler(Rc::new(JsDocTypeChecker));
        context.register_handler(Rc::new(VarKeywordChecker));
        context.register_handler(Rc::new(VariableNameChecker));
        context.register_handler(Rc::new(FunctionNameChecker));
        context.register_handler(Rc::new(FunctionJsDocChecker));
        context.register_handler(Rc::new(UnusedVariableChecker));
        // TODO Handle multiple files in case of unused functionchecker
        // context.register_handler(Rc::new(UnusedFunctionChecker));

        let result = context.run();
        allocator.reset();

        match result {
            Ok(errored) => {
                if !errored {
                    let message = format!("{file_name}: ✔ Minden teszt lefutott sikeresen (:");
                    println!("{}", message.green());
                    files_errored.push((file_name, errored));
                } else {
                    file_errored_flag = true;
                    files_errored.push((file_name, errored));
                }
            }
            Err(message) => {
                ErrorHandler::print_error(message);
                exit(1);
            }
        }
    }

    println!();
    for (file_name, flag) in files_errored {
        if !flag {
            let message = format!("{} ✔ Sikeres", file_name);
            println!("{}", message.green());
            continue
        }
        let message = format!("{} Sikertelen", file_name);
        ErrorHandler::print_error(message);
    }

    if file_errored_flag {
        exit(1);
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
                        return Err(format!(
                            "nem futtattad a git add parancsot miutan modositottad a kovetkezo fajlt: {filename}",
                        ));
                    }
                }
                Err(_) => {
                    return Err("nem sikerult a modositott fajlok lekerese".to_string());
                }
            }
        }
    } else {
        return Err("nem sikerult a git staged fajlok lekerese".to_string());
    }

    Ok(staged_files)
}