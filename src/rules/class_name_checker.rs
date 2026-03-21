use oxc::ast::AstKind;
use rust_i18n::t;

use crate::{
    api::{Handler, HandlerResult},
    rules::variable_name_checker::contains_number_or_hungarian_letter,
};

pub struct ClassNameChecker;

impl Handler for ClassNameChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        for node in semantic.nodes() {
            let AstKind::Class(class) = node.kind() else {
                continue;
            };

            let Some(binding_identifier) = &class.id else {
                errors.push(t!("C01", line = context.get_line(class.span.start)).to_string());
                continue;
            };
            let name = binding_identifier.name;
            let start = binding_identifier.span.start;
            if name.len() < 5 {
                errors.push(
                    t!(
                        "C02",
                        line = context.get_line(start),
                        class = context.lines[context.get_line(start) - 1],
                        highlight = format!(
                            "{}{}",
                            " ".repeat(context.get_column(start) - 1),
                            "^".repeat(name.len())
                        )
                    )
                    .to_string(),
                );
            }

            if contains_number_or_hungarian_letter(name.as_str()) {
                errors.push(
                    t!(
                        "C03",
                        line = context.get_line(start),
                        class = context.lines[context.get_line(start) - 1],
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
        t!("SCM02").to_string()
    }

    fn title(&self) -> String {
        t!("TT02").to_string()
    }
}
