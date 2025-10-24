use crate::api::Handler;

pub struct CommentChecker;

impl Handler for CommentChecker {
    fn handle(&self, context: &mut crate::api::Context) -> Result<(), String> {
        let mut in_jsdoc = false;
        for (line_number, line) in context.file_contents.iter().enumerate() {
            let trimmed = line.trim();
            if in_jsdoc {
                continue;
            }

            if trimmed.contains("/**") {
                in_jsdoc = true;
            }
            if trimmed.contains("*/") {
                in_jsdoc = false
            }

            // Skip empty or comment-only lines
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            if trimmed.chars().any(|c| c.is_alphanumeric()) && !trimmed.contains("//") {
                return context.end_of_handle(Some(
                    format!(
                        "Nincs komment kodsor: {} mert '{}' sorban nem irtal magyarazatot",
                        line_number + 1,
                        trimmed
                    )
                    .as_str(),
                ));
            }
        }
        context.end_of_handle(None)
    }
    fn title(&self) -> String {
        format!("Sorvegi kommentek ellenorzese")
    }

    fn success_message(&self) -> String {
        format!("Sorvegi komment jo")
    }
}
