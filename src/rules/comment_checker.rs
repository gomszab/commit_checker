use crate::api::{Handler, HandlerResult};

pub struct CommentChecker;

impl Handler for CommentChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        let mut in_jsdoc = false;
        for (line_number, line) in context.lines.iter().enumerate() {
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
                    "sor: {}: Nincs comment a sor végén\n{}\n{}",
                    line_number + 1,
                    trimmed,
                    "^".repeat(trimmed.len())
                ));
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }
    fn title(&self) -> String {
        format!("Sorvegi kommentek ellenorzese")
    }

    fn success_message(&self) -> String {
        format!("Sorvegi komment jo")
    }
}
