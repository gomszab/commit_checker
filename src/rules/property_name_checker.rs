use oxc::ast::{AstKind, ast::PropertyKey};

use crate::{
    api::{Handler, HandlerResult},
    rules::variable_name_checker::contains_number_or_hungarian_letter,
};

pub struct PropertyNameChecker;

impl Handler for PropertyNameChecker {
    fn handle<'a>(&self, context: &'a crate::api::FileContext<'a>) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        let nodes = semantic.nodes();
        for node in nodes {
            let AstKind::PropertyDefinition(def) = node.kind() else {
                continue;
            };

            let (name, start) = match &def.key {
                PropertyKey::StaticIdentifier(id_name) => {
                    (id_name.name.as_str(), id_name.span.start)
                }
                PropertyKey::PrivateIdentifier(id_name) => {
                    (&id_name.name.as_str()[0..], id_name.span.start + 1)
                }
                _ => continue,
            };

            if name.len() < 5 {
                errors.push(format!(
                    "sor: {}: A property neveknek legalább 5 karakter hosszúnak kell lenniük\n{}\n{}",
                    context.get_line(start),
                    context.lines[context.get_line(start) - 1],
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(start) - 1),
                        "^".repeat(name.len())
                    )
                ));
            }

            if contains_number_or_hungarian_letter(name) {
                errors.push(format!(
                            "sor: {}: A property név számot vagy ékezetes karaktert tartalmaz, ami rontja az olvashatóságot\n{}\n{}",
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
        format!("Property nevek rendben")
    }
    fn title(&self) -> String {
        format!("Property nevek ellenőrzése...")
    }
}
