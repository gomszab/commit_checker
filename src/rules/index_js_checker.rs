use crate::api::{Handler, HandlerResult};

pub struct IndexJsChecker;

impl Handler for IndexJsChecker {
    fn handle(&self, context: &mut crate::api::Context) -> HandlerResult {
        let contains_index = context.staged_files.iter().any(|f| f == "index.js");
        if !contains_index {
            return HandlerResult::FatalError(
                "Nincs hozzaadva az index.js a commithoz".to_string(),
            );
        }

        HandlerResult::Ok
    }

    fn title(&self) -> String {
        format!("Commit ellenorzese folyamatban")
    }

    fn success_message(&self) -> String {
        format!("A commit jonak tunik")
    }
}
