use std::{fs, thread, time::Duration};

use crate::api::Handler;

pub struct FileContentRead;

impl Handler for FileContentRead {
    fn handle(&self, context: &mut crate::api::Context) -> Result<(), String> {
        thread::sleep(Duration::from_secs(2));
        let file = context
            .staged_files
            .iter()
            .find(|filename| filename.ends_with("index.js"));
        if let Some(file_path) = file {
            if let Ok(content) = fs::read_to_string(file_path) {
                content.lines().for_each(|li| context.add_file_contents(li));
                context.end_of_handle(None)
            } else {
                context.end_of_handle(Some(format!("Az index.js file nem olvashato").as_str()))
            }
        } else {
            context.end_of_handle(Some(format!("A commitban nincs index.js").as_str()))
        }
    }

    fn title(&self) -> String {
        format!("Index.js file megnyitasa folyamatban")
    }

    fn success_message(&self) -> String {
        format!("Fajl beolvasva")
    }
}
