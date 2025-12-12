use crate::api::{Handler, HandlerResult};

pub struct CommentChecker;

impl Handler for CommentChecker {
    fn handle(&self, context: &mut crate::api::Context) -> HandlerResult {
        let mut errors = Vec::new();
        let mut in_jsdoc = false;
        for (line_number, line) in context.file_contents.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.contains("/**") {
                in_jsdoc = true;
            }

            if in_jsdoc {
                if trimmed.contains("*/") {
                    in_jsdoc = false
                }
                continue;
            }

            // Skip empty or comment-only lines
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            if trimmed.chars().any(|c| c.is_alphanumeric()) && !trimmed.contains("//") {
                errors.push(format!(
                    "Nincs komment kodsor: {} mert '{}' sorban nem irtal magyarazatot",
                    line_number + 1,
                    trimmed
                ));
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::SoftErrors(errors)
        }
    }
    fn title(&self) -> String {
        format!("Sorvegi kommentek ellenorzese")
    }

    fn success_message(&self) -> String {
        format!("Sorvegi komment jo")
    }
}
