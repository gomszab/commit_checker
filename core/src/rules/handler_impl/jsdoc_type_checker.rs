use commit_checker_message_handler::{info_message, rule_success_message, validation_error};

use crate::rules::api::{Handler, HandlerResult};

pub struct JsDocTypeChecker;

// TODO: maybe get it from a config file?
const FORBIDDEN_TYPES: [&str; 3] = ["Object", "Array", "*"];

impl Handler for JsDocTypeChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let semantic = context.semantic.get().unwrap();

        for jsdoc in semantic.jsdoc().iter_all() {
            for tag in jsdoc.tags() {
                let Some(type_part) = tag.r#type() else {
                    continue;
                };

                let ty = type_part.parsed();
                let Some(found_forbidden) = FORBIDDEN_TYPES.iter().find(|s| ty.contains(*s)) else {
                    continue;
                };

                validation_error!(
                    &mut ioc.message_handler,
                    "D01",
                    &context.file_name,
                    context.get_line(tag.span.start) as usize,
                    context.get_line(tag.span.start) as usize,
                    context.lines[context.get_line(jsdoc.span.end - 2)].len() as usize,
                    jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                        ..=context.get_line(jsdoc.span.end) - 1]
                        .to_vec()
                        .join("\n"),
                    forbidden_type = found_forbidden,
                );
            }
        }

        HandlerResult::Ok
    }

    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM09");
    }

    fn title(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        info_message!(&mut ioc.message_handler, "TT09");
    }
}
