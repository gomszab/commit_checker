use commit_checker_message_handler::{rule_success_message, validation_error};
use oxc::ast::{AstKind, ast::BindingPatternKind};

use crate::{
    handler_impl::check_helper::contains_number_or_hungarian_letter,
    rules::api::{Handler, HandlerResult},
};

pub struct VariableNameChecker;

impl Handler for VariableNameChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        let semantic = context.semantic.get().unwrap();
        for node in semantic.nodes() {
            if let AstKind::VariableDeclaration(decl) = node.kind() {
                // There can be multiple declarations in a single line.
                for var in &decl.declarations {
                    if let BindingPatternKind::BindingIdentifier(identifier) = &var.id.kind {
                        let name = identifier.name;
                        let start = var.span.start;

                        if name.len() < 5 {
                            validation_error!(
                                &mut ioc.message_handler,
                                "V03",
                                &context.file_name,
                                context.get_line(start) as usize,
                                context.get_column(start) - 1,
                                context.get_column(start) + name.len() as usize,
                                variable = context.lines[context.get_line(start) - 1],
                            );
                            result = HandlerResult::Error;
                        }

                        if contains_number_or_hungarian_letter(name.as_str()) {
                            validation_error!(
                                &mut ioc.message_handler,
                                "V04",
                                &context.file_name,
                                context.get_line(start) as usize,
                                context.get_column(start) - 1,
                                context.get_column(start) + name.len() as usize,
                                variable = context.lines[context.get_line(start) - 1],
                            );
                            result = HandlerResult::Error;
                        }
                    }
                }
            }
        }

        result
    }

    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM15");
    }

    fn code(&self) -> &'static str {
        "TT15"
    }
}

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_v03 => ("TT15/small_variable_name.js", VariableNameChecker, "V03", Error),
       test_v04_hun => ("TT15/variable_name_hun.js", VariableNameChecker, "V04", Error),
       test_v04_num => ("TT15/variable_name_num.js", VariableNameChecker, "V04", Error),
       test_valid => ("TT15/valid.js", VariableNameChecker, "", Ok),
    }
}
