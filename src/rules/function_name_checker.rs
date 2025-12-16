use oxc::ast::ast::Statement;

use crate::{
    api::{Handler, HandlerResult},
    rules::variable_name_checker::contains_number_or_hungarian_letter,
};

pub struct FunctionNameChecker;

impl Handler for FunctionNameChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        for declaration in context.program.body.iter() {
            if let Statement::FunctionDeclaration(decl) = declaration {
                let Some(identifier) = &decl.id else {
                    continue;
                };
                let name = identifier.name;
                let start = identifier.span.start;

                if name.len() < 5 {
                    errors.push(format!(
                            "sor: {}: A függvényneveknek legalább 5 karakter hosszúnak kell lenniük\n{}\n{}",
                            context.get_line(start),
                            context.lines[context.get_line(start) - 1],
                            format!("{}{}", " ".repeat(context.get_column(start) - 1), "^".repeat(name.len()))
                        ));
                }

                if contains_number_or_hungarian_letter(name.as_str()) {
                    errors.push(format!(
                            "sor: {}: A függvénynév számot vagy ékezetes karaktert tartalmaz, ami rontja az olvashatóságot\n{}\n{}",
                            context.get_line(decl.span.start),
                            context.lines[context.get_line(start) - 1],
                            format!("{}{}", " ".repeat(context.get_column(start) - 1), "^".repeat(name.len()))
                        ));
                }
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
