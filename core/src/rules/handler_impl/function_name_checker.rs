use commit_checker_message_handler::{info_message, rule_success_message, validation_error};
use oxc::ast::{
    AstKind,
    ast::{BindingIdentifier, IdentifierName},
};
use oxc_semantic::AstNodes;

use crate::{
    handler_impl::check_helper::contains_number_or_hungarian_letter,
    rules::api::{Handler, HandlerResult},
};

pub struct FunctionNameChecker;

impl Handler for FunctionNameChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        let semantic = context.semantic.get().unwrap();
        for (name, start) in get_all_func_names_and_spans(semantic.nodes(), ioc, &context) {
            if name.len() < 5 {
                validation_error!(
                    &mut ioc.message_handler,
                    "F01",
                    &context.file_name,
                    context.get_line(start) as usize,
                    (context.get_column(start) - 1) as usize,
                    (context.get_column(start) + name.len()) as usize,
                    function = context.lines[context.get_line(start) - 1],
                );
                result = HandlerResult::Error;
            }

            if contains_number_or_hungarian_letter(&name) {
                validation_error!(
                    &mut ioc.message_handler,
                    "F02",
                    &context.file_name,
                    context.get_line(start) as usize,
                    (context.get_column(start) - 1) as usize,
                    (context.get_column(start) + name.len()) as usize, //TODO Are the Hungarian letters skiped at the length determination?
                    function = context.lines[context.get_line(start) - 1],
                );
                result = HandlerResult::Error;
            }
        }

        result
    }

    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM05");
    }

    fn code(&self) -> &'static str {
        "TT05"
    }
}

/// Gets the names of all functions in the file and the starts of them.
fn get_all_func_names_and_spans<'a>(
    nodes: &'a AstNodes,
    ioc: &mut crate::api::CommitCheckerIoC,
    context: &'a crate::rules::api::FileContext<'a>,
) -> Vec<(oxc::span::Atom<'a>, u32)> {
    let mut nodes = nodes.iter();
    let mut names = Vec::new();
    while let Some(node) = nodes.next() {
        match node.kind() {
            AstKind::Function(func) => {
                let Some(BindingIdentifier {
                    span,
                    name,
                    symbol_id: _,
                }) = func.id
                else {
                    continue;
                };

                names.push((name, span.start));
            }
            AstKind::MethodDefinition(_) => {
                // We have to do this, because for some reason the id is after the
                // MethodDefinition.
                if let Some(node) = nodes.next() {
                    let AstKind::IdentifierName(IdentifierName { span, name }) = node.kind() else {
                        continue;
                    };
                    names.push((*name, span.start));
                } else {
                    validation_error!(
                        &mut ioc.message_handler,
                        "F04",
                        &context.file_name,
                        0_usize,
                        0_usize,
                        1_usize,
                    );
                }
            }
            _ => continue,
        }
    }

    names
}

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_f01 => ("TT05/small_function_name.js", FunctionNameChecker, "F01", Error),
       test_f02_num=> ("TT05/function_name_num.js", FunctionNameChecker, "F02", Error),
       test_f02_hun => ("TT05/function_name_hun.js", FunctionNameChecker, "F02", Error),
    //    test_f04 => ("TT05/function_no_name.js", FunctionNameChecker, "F04", Error),
       test_valid => ("TT05/valid.js", FunctionNameChecker, "", Ok),
    }
}
