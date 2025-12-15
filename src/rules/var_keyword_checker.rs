use oxc::ast::ast::{Statement, VariableDeclarationKind};

use crate::api::{Handler, HandlerResult};

pub struct VarKeywordChecker;

impl Handler for VarKeywordChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        for declaration in context.program.body.iter() {
            if let Statement::VariableDeclaration(decl) = declaration
                && let VariableDeclarationKind::Var = decl.kind
            {
                errors.push(format!(
                    "sor: {}: ne használj var-t!!!!",
                    context.get_line(decl.span.start)
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
        format!("Nincs var")
    }

    fn title(&self) -> String {
        format!("Var használat ellenőrzése...")
    }
}
