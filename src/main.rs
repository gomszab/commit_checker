mod api;
mod rules;

use std::process::{Command, exit};
use std::rc::Rc;

use colored::Colorize;
use oxc::allocator::Allocator;

use crate::api::FileContext;
use crate::rules::{
    CommentChecker, FunctionNameChecker, JsDocTypeChecker, TypedefJsDocChecker, VarKeywordChecker,
    VariableJsDocChecker, VariableNameChecker,
};

fn main() {
    let files = match get_staged_files() {
        Ok(files) => files,
        Err(message) => {
            eprintln!("{}", message.red());
            exit(1);
        }
    };

    // Needed for oxc.
    let mut allocator = Allocator::new();

    let mut file_errored = false;
    for file_name in files {
        let content = match std::fs::read_to_string(&file_name) {
            Ok(content) => content,
            Err(_) => {
                let message = format!(
                    "nem sikerült a {file_name} fájl olvasása\nelképzelhető hogy stagelve van egy fájl, amit kitöröltél; nézd meg a git status-t, és ha zölddel ott van egy fájl, ami törölve van, futtasd a git rm --cached {file_name} parancsot)"
                );
                eprintln!("{}", message.red());
                exit(1);
            }
        };

        let mut context = match FileContext::new(file_name.clone(), &content, &allocator) {
            Ok(context) => context,
            Err(message) => {
                eprintln!("{}", message.red());
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

        let result = context.run();
        allocator.reset();

        match result {
            Ok(errored) => {
                if errored {
                    file_errored = true;
                }
                let message = format!("{file_name}: ✔ Minden teszt lefutott sikeresen (:");
                println!("{}", message.green());
            }
            Err(message) => {
                eprintln!("{}", message.red());
                exit(1);
            }
        }
    }

    if file_errored {
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

    if staged_files.iter().any(|f| f.ends_with(".js")) {
        Ok(staged_files)
    } else {
        Err("Nincs .js fájl hozzáadva a commithoz".to_string())
    }
}
