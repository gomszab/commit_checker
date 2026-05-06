use commit_checker_message_handler::{rule_success_message, validation_error};

use crate::rules::api::{Handler, HandlerResult};

pub struct TypeJsDocChecker;

impl Handler for TypeJsDocChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        let semantic = context.semantic.get().unwrap();

        for jsdoc in semantic.jsdoc().iter_all() {
            if let Some(tag) = jsdoc.tags().iter().find(|tag| tag.kind.parsed() == "type") {
                let type_comment = tag.type_comment();
                let ty = type_comment.0;
                let comment = type_comment.1.parsed();

                let start = tag.span.start;
                let line_num = context.get_line(start);
                let comment_span = type_comment.1.span;
                if ty.is_none() && comment.is_empty() {
                    validation_error!(
                        &mut ioc.message_handler,
                        "TD01",
                        &context.file_name,
                        line_num as usize,
                        0_usize,
                        (context.get_column(comment_span.start)) as usize,
                        jsdoc = context.lines[line_num - 1],
                    );
                    result = HandlerResult::Error;
                    continue;
                } else if type_comment.0.is_none() {
                    validation_error!(
                        &mut ioc.message_handler,
                        "TD02",
                        &context.file_name,
                        line_num as usize,
                        0_usize,
                        context.lines[line_num - 1].len() + 1,
                        jsdoc = context.lines[line_num - 1],
                    );
                    result = HandlerResult::Error;
                    continue;
                } else if comment.is_empty() {
                    validation_error!(
                        &mut ioc.message_handler,
                        "TD03",
                        &context.file_name,
                        line_num as usize,
                        0_usize,
                        context.lines[line_num - 1].len() + 1,
                        jsdoc = context.lines[line_num - 1],
                    );
                    result = HandlerResult::Error;
                    continue;
                }
            }
        }
        result
    }

    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM09");
    }

    fn code(&self) -> &'static str {
        "TT09"
    }
}

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_td01 => ("TT09/no_type_no_desc.js", TypeJsDocChecker, "TD01", Error),
       test_td02 => ("TT09/no_type.js", TypeJsDocChecker, "TD02", Error),
       test_td03 => ("TT09/no_desc.js", TypeJsDocChecker, "TD03", Error),
       test_valid => ("TT09/valid.js", TypeJsDocChecker, "", Ok),
    }
}
