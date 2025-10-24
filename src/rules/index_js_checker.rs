use crate::api::Handler;

pub struct IndexJsChecker;

impl Handler for IndexJsChecker {
    fn handle(&self, context: &mut crate::api::Context) -> Result<(), String> {
        let contains_index = context.staged_files.iter().any(|f| f == "index.js");
        if !contains_index {
            return context.end_of_handle(Some("Nincs hozzaadva az index.js a commithoz"));
        }
        context.end_of_handle(None)
    }

    fn title(&self) -> String {
        format!("Commit ellenorzese folyamatban")
    }

    fn success_message(&self) -> String {
        format!("A commit jonak tunik")
    }
}
