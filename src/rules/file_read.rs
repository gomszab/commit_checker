use std::{fs, thread, time::Duration};

use crate::api::{Handler, HandlerResult};

pub struct FileContentRead;

impl Handler for FileContentRead {
    fn handle(&self, context: &mut crate::api::Context) -> HandlerResult {
        let file = context
            .staged_files
            .iter()
            .find(|filename| filename.ends_with("index.js"));
        if let Some(file_path) = file {
            if let Ok(content) = fs::read_to_string(file_path) {
                content.lines().for_each(|li| context.add_file_contents(li));
                return HandlerResult::Ok;
            } else {
                return HandlerResult::FatalError(format!("Az index.js file nem olvashato"));
            }
        } else {
            return HandlerResult::FatalError(format!("A commitban nincs index.js"));
        }
    }

    fn title(&self) -> String {
        format!("Index.js file megnyitasa folyamatban")
    }

    fn success_message(&self) -> String {
        format!("Fajl beolvasva")
    }
}
