use oxc::ast::ast::{Statement, VariableDeclarationKind};
use rust_i18n::t;

use crate::api::{Handler, HandlerResult};

pub struct VarKeywordChecker;

impl Handler for VarKeywordChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        for declaration in context.program.body.iter() {
            if let Statement::VariableDeclaration(decl) = declaration
                && let VariableDeclarationKind::Var = decl.kind
            {
                errors.push(t!(
                    "V02", line =
                    context.get_line(decl.span.start)
                ).to_string());
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self) -> String {
       t!("SCM13").to_string()
    }

    fn title(&self) -> String {
        t!("TT13").to_string()
    }
}
