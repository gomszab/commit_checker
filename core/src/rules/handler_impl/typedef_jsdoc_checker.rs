use commit_checker_message_handler::{info_message, rule_success_message, validation_error};

use crate::rules::api::{Handler, HandlerResult};

pub struct TypedefJsDocChecker;

impl Handler for TypedefJsDocChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let semantic = context.semantic.get().unwrap();

        for jsdoc in semantic.jsdoc().iter_all() {
            #[allow(for_loops_over_fallibles)]
            for tag in jsdoc
                .tags()
                .iter()
                .find(|tag| tag.kind.parsed() == "typedef")
            {
                let (type_part, name_part) = tag.type_comment();

                if let None = type_part
                    && name_part.parsed().len() == 0
                {
                    validation_error!(
                        &mut ioc.message_handler,
                        "TD04",
                        &context.file_name,
                        context.get_line(tag.span.start) as usize,
                        0_usize,
                        context.lines[context.get_line(jsdoc.span.end - 2)].len() as usize,
                        jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                    );
                    continue;
                } else if name_part.parsed().len() == 0 {
                    validation_error!(
                        &mut ioc.message_handler,
                        "TD05",
                        &context.file_name,
                        context.get_line(tag.span.start) as usize,
                        0_usize,
                        context.lines[context.get_line(jsdoc.span.end - 2)].len() as usize,
                        jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                    );
                    continue;
                } else if let None = type_part {
                    validation_error!(
                        &mut ioc.message_handler,
                        "TD06",
                        &context.file_name,
                        context.get_line(tag.span.start) as usize,
                        0_usize,
                        context.lines[context.get_line(jsdoc.span.end - 2)].len() as usize,
                        jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                    );
                    continue;
                }
            }
        }

        HandlerResult::Ok
    }

    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM10");
    }

    fn title(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        info_message!(&mut ioc.message_handler, "TT10");
    }
}
