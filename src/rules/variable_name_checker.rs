use oxc::ast::{
    AstKind,
    ast::{BindingPatternKind, Statement},
};

use crate::api::{Handler, HandlerResult};

pub struct VariableNameChecker;

impl Handler for VariableNameChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        for node in semantic.nodes() {
            if let AstKind::VariableDeclaration(decl) = node.kind() {
                // There can be multiple declarations in a single line.
                for var in &decl.declarations {
                    if let BindingPatternKind::BindingIdentifier(identifier) = &var.id.kind {
                        let name = identifier.name;
                        println!("{}", name);
                        let start = var.span.start;

                        if name.len() < 5 {
                            errors.push(format!(
                            "sor: {}: A változóneveknek legalább 5 karakter hosszúnak kell lenniük\n{}\n{}",
                            context.get_line(start),
                            context.lines[context.get_line(start) - 1],
                            format!("{}{}", " ".repeat(context.get_column(start) - 1), "^".repeat(name.len()))
                        ));
                        }

                        if contains_number_or_hungarian_letter(name.as_str()) {
                            errors.push(format!(
                            "sor: {}: A változónév számot vagy ékezetes karaktert tartalmaz, ami rontja az olvashatóságot\n{}\n{}",
                            context.get_line(start),
                            context.lines[context.get_line(start) - 1],
                            format!("{}{}", " ".repeat(context.get_column(start) - 1), "^".repeat(name.len()))
                        ));
                        }
                    }
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
        format!("Változónevek rendben")
    }

    fn title(&self) -> String {
        format!("Változónevek ellenőrzése...")
    }
}

pub fn contains_number_or_hungarian_letter(s: &str) -> bool {
    let hungarian_special_chars = [
        'á', 'é', 'í', 'ó', 'ö', 'ő', 'ú', 'ü', 'ű', 'Á', 'É', 'Í', 'Ó', 'Ö', 'Ő', 'Ú', 'Ü', 'Ű',
    ];

    s.chars()
        .any(|c| c.is_numeric() || hungarian_special_chars.contains(&c))
}
