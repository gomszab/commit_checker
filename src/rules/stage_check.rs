use std::process::Command;

use crate::api::{Handler, HandlerResult};

pub struct StageHandler;

impl Handler for StageHandler {
    fn handle(&self, context: &mut crate::api::Context) -> HandlerResult {
        let output = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .output();
        if let Ok(content) = output {
            let files = String::from_utf8_lossy(&content.stdout);

            for filename in files.lines() {
                context.add_staged_files(filename);
                let diff_output = Command::new("git")
                    .args(["diff", "--name-only", filename])
                    .output();
                match diff_output {
                    Ok(diff_content) => {
                        if !diff_content.stdout.is_empty() {
                            return HandlerResult::FatalError(format!(
                                "nem futtattad a git add parancsot miutan modositottad a kovetkezo fajlt: {}",
                                filename
                            ));
                        }
                    }
                    Err(_) => {
                        return HandlerResult::FatalError(format!(
                            "nem sikerult a modositott fajlok lekerese"
                        ));
                    }
                }
            }
            HandlerResult::Ok
        } else {
            HandlerResult::FatalError(format!("nem sikerult a git staged fajlok lekerese"))
        }
    }
    fn title(&self) -> String {
        format!("git parancs futtatasa...")
    }

    fn success_message(&self) -> String {
        format!("A git mukodik")
    }
}
