use commit_checker_message_handler::{info_message, rule_success_message, validation_error};
use oxc::ast::AstKind;

use crate::{
    handler_impl::check_helper::contains_number_or_hungarian_letter,
    rules::api::{Handler, HandlerResult},
};

pub struct ClassNameChecker;

impl Handler for ClassNameChecker {
    fn handle(
        &self,
        context: &crate::rules::api::FileContext,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        let semantic = context.semantic.get().unwrap();
        for node in semantic.nodes() {
            let AstKind::Class(class) = node.kind() else {
                continue;
            };

            let Some(binding_identifier) = &class.id else {
                validation_error!(
                    &mut ioc.message_handler,
                    "C01",
                    &context.file_name,
                    context.get_line(class.span.start) as usize,
                    context.get_column(class.span.start) - 1,
                    context.get_column(class.span.end)
                );
                result = HandlerResult::Error;
                continue;
            };
            let name = binding_identifier.name;
            let start = binding_identifier.span.start;
            if name.len() < 5 {
                validation_error!(
                    &mut ioc.message_handler,
                    "C02",
                    &context.file_name,
                    context.get_line(class.span.start) as usize,
                    context.get_column(start) - 1,
                    context.get_column(start) + name.len(),
                    class = context.lines[context.get_line(start) - 1],
                );
                result = HandlerResult::Error;
            }

            if contains_number_or_hungarian_letter(name.as_str()) {
                validation_error!(
                    &mut ioc.message_handler,
                    "C03",
                    &context.file_name,
                    context.get_line(class.span.start) as usize,
                    context.get_column(start) - 1,
                    context.get_column(start) - 1 + name.len(),
                    class = context.lines[context.get_line(start) - 1],
                );
                result = HandlerResult::Error;
            }
        }

        result
    }

    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM02");
    }

    fn code(&self) -> &'static str {
        "TT02"
    }
}

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_c01 => ("TT02/no_class_name.js", ClassNameChecker, "C01", Error),
       test_c02 => ("TT02/small_class_name.js", ClassNameChecker, "C02", Error),
       test_c03_number => ("TT02/classname_number.js", ClassNameChecker, "C03", Error),
       test_c03_hun => ("TT02/classname_hungarian.js", ClassNameChecker, "C03", Error),
       test_valid => ("TT02/valid.js", ClassNameChecker, "", Ok),
    }
}
