use oxc::ast::{
    AstKind,
    ast::{BindingIdentifier, IdentifierName},
};
use oxc_semantic::AstNodes;
use rust_i18n::t;

use crate::{
    api::{Handler, HandlerResult},
    rules::variable_name_checker::contains_number_or_hungarian_letter,
};

pub struct FunctionNameChecker;

impl Handler for FunctionNameChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = &context.semantic;
        for (name, start) in get_all_func_names_and_spans(semantic.nodes()) {
            if name.len() < 5 {
                errors.push(
                    t!(
                        "F01",
                        line = context.get_line(start),
                        function = context.lines[context.get_line(start) - 1],
                        highlight = format!(
                            "{}{}",
                            " ".repeat(context.get_column(start) - 1),
                            "^".repeat(name.len())
                        )
                    )
                    .to_string(),
                );
            }

            if contains_number_or_hungarian_letter(&name) {
                errors.push(
                    t!(
                        "F02",
                        line = context.get_line(start),
                        function = context.lines[context.get_line(start) - 1],
                        highlight = format!(
                            "{}{}",
                            " ".repeat(context.get_column(start) - 1),
                            "^".repeat(name.len())
                        )
                    )
                    .to_string(),
                );
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self) -> String {
        t!("SCM05").to_string()
    }

    fn title(&self) -> String {
        t!("TT05").to_string()
    }
}

/// Gets the names of all functions in the file and the starts of them.
fn get_all_func_names_and_spans<'a>(nodes: &'a AstNodes) -> Vec<(oxc::span::Atom<'a>, u32)> {
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
                let AstKind::IdentifierName(IdentifierName { span, name }) =
                    nodes.next().expect(&t!("F04").to_string()).kind()
                else {
                    continue;
                };

                names.push((*name, span.start));
            }
            _ => continue,
        }
    }

    names
}
