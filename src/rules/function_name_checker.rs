use oxc::ast::{
    AstKind,
    ast::{BindingIdentifier, IdentifierName},
};
use oxc_semantic::AstNodes;

use crate::{
    api::{Handler, HandlerResult},
    rules::variable_name_checker::contains_number_or_hungarian_letter,
};

pub struct FunctionNameChecker;

impl Handler for FunctionNameChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        for (name, start) in get_all_func_names_and_spans(semantic.nodes()) {
            if name.len() < 5 {
                errors.push(format!(
                            "sor: {}: A függvényneveknek legalább 5 karakter hosszúnak kell lenniük\n{}\n{}",
                            context.get_line(start),
                            context.lines[context.get_line(start) - 1],
                            format!("{}{}", " ".repeat(context.get_column(start) - 1), "^".repeat(name.len()))
                        ));
            }

            if contains_number_or_hungarian_letter(&name) {
                errors.push(format!(
                            "sor: {}: A függvénynév számot vagy ékezetes karaktert tartalmaz, ami rontja az olvashatóságot\n{}\n{}",
                            context.get_line(start),
                            context.lines[context.get_line(start) - 1],
                            format!("{}{}", " ".repeat(context.get_column(start) - 1), "^".repeat(name.len()))
                        ));
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self) -> String {
        format!("Függvénynevek rendben")
    }

    fn title(&self) -> String {
        format!("Függvénynevek ellenőrzése...")
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
                    nodes.next().expect("nincs a metódusnak neve").kind()
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
