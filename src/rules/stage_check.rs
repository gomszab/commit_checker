use std::process::Command;

use crate::api::Handler;

pub struct StageHandler;

impl Handler for StageHandler {
    fn handle(&self, context: &mut crate::api::Context) -> Result<(), String> {
        let output = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .output();
        if let Ok(content) = output {
            let files = String::from_utf8_lossy(&content.stdout);
            files
                .lines()
                .for_each(|filename| context.add_staged_files(filename));
            context.end_of_handle(None)
        } else {
            context.end_of_handle(Some(
                format!("nem sikerult a git staged fajlok lekerese").as_str(),
            ))
        }
    }
    fn title(&self) -> String {
        format!("git parancs futtatasa...")
    }

    fn success_message(&self) -> String {
        format!("A git mukodik")
    }
}
