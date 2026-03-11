use oxc::ast::AstKind;

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
                errors.push(format!(
                    "sor: {}: A classnak nincs neve",
                    context.get_line(class.span.start)
                ));
                continue;
            };
            let name = binding_identifier.name;
            let start = binding_identifier.span.start;
            if name.len() < 5 {
                errors.push(format!(
                    "sor: {}: A class neveknek legalább 5 karakter hosszúnak kell lenniük\n{}\n{}",
                    context.get_line(start),
                    context.lines[context.get_line(start) - 1],
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(start) - 1),
                        "^".repeat(name.len())
                    )
                ));
            }

            if contains_number_or_hungarian_letter(name.as_str()) {
                errors.push(format!(
                            "sor: {}: A class név számot vagy ékezetes karaktert tartalmaz, ami rontja az olvashatóságot\n{}\n{}",
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
        format!("Class nevek rendben")
    }

    fn title(&self) -> String {
        format!("Class nevek ellenőrzése...")
    }
}
