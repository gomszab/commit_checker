use commit_checker_message_handler::{info_message, rule_success_message, validation_error};

use crate::rules::api::{Handler, HandlerResult};

pub struct CommentChecker;

impl Handler for CommentChecker {
    fn handle(
        &self,
        context: &crate::rules::api::FileContext,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
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
                validation_error!(
                    &mut ioc.message_handler,
                    "COM01",
                    &context.file_name,
                    line_number + 1 as usize,
                    determine_start(line),
                    line.len(),
                    comment = line
                );

                result = HandlerResult::Error;
            }
        }

        result
    }
    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM03");
    }

    fn title(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        info_message!(&mut ioc.message_handler, "TT03");
    }
}

fn determine_start(line: &str) -> usize {
    if let Some((index, _)) = line.char_indices().find(|&(_, c)| !c.is_whitespace()) {
        index
    } else {
        0_usize
    }
}

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_com01 => ("TT03/no_comment.js", CommentChecker, "COM01", Error),
       test_valid => ("TT03/valid.js", CommentChecker, "", Ok),
    }
}
