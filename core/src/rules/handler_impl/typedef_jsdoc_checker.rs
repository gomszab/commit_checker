use commit_checker_message_handler::{rule_success_message, validation_error};

use crate::rules::api::{Handler, HandlerResult};

pub struct TypedefJsDocChecker;

impl Handler for TypedefJsDocChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
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
                        "TD04",
                        context.get_line(tag.span.start) as usize,
                        0_usize,
                        context.lines[context.get_line(tag.span.start) - 1].len() + 1 as usize,
                        jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                    );
                    result = HandlerResult::Error;
                    continue;
                } else if name_part.parsed().len() == 0 {
                    validation_error!(
                        "TD05",
                        context.get_line(tag.span.start) as usize,
                        0_usize,
                        context.lines[context.get_line(tag.span.start) - 1].len() + 1 as usize,
                        jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                    );
                    result = HandlerResult::Error;
                    continue;
                } else if let None = type_part {
                    validation_error!(
                        "TD06",
                        context.get_line(tag.span.start) as usize,
                        0_usize,
                        context.lines[context.get_line(tag.span.start) - 1].len() + 1 as usize,
                        jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                    );
                    result = HandlerResult::Error;
                    continue;
                }
            }
        }

        result
    }

    fn success_message(&self) {
        rule_success_message!("SCM10");
    }

    fn code(&self) -> &'static str {
        "TT10"
    }
}

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_td04 => ("TT10/typedef_noname_notype.js", TypedefJsDocChecker, "TD04", Error),
       test_td05 => ("TT10/typedef_noname.js", TypedefJsDocChecker, "TD05", Error),
       test_td06 => ("TT10/typedef_notype.js", TypedefJsDocChecker, "TD06", Error),
       test_valid => ("TT10/valid.js", TypedefJsDocChecker, "", Ok),
    }
}
