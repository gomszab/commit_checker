use crate::{
    api::{Handler, HandlerResult},
    rules::jsdoc_counter::is_definition_line,
};

pub struct VarKeywordChecker;

impl Handler for VarKeywordChecker {
    fn handle(&self, context: &mut crate::api::Context) -> HandlerResult {
        let mut errors = Vec::new();
        for (line_number, line) in context.file_contents.iter().enumerate() {
            if is_definition_line(line) && line.starts_with("var") {
                errors.push(format!("sor: {}: ne használj var-t!!!!", line_number + 1));
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::SoftErrors(errors)
        }
    }

    fn success_message(&self) -> String {
        format!("Nincs var")
    }

    fn title(&self) -> String {
        format!("Var használat ellenőrzése...")
    }
}
