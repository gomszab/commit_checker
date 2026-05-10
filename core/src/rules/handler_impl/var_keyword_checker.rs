use commit_checker_message_handler::{rule_success_message, validation_error};
use oxc::ast::ast::{Statement, VariableDeclarationKind};

use crate::rules::api::{Handler, HandlerResult};

pub struct VarKeywordChecker;

impl Handler for VarKeywordChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        for declaration in context.program.body.iter() {
            if let Statement::VariableDeclaration(decl) = declaration
                && let VariableDeclarationKind::Var = decl.kind
            {
                validation_error!(
                    "V02",
                    context.get_line(decl.span.start) as usize,
                    0_usize,
                    1_usize as usize
                );
                result = HandlerResult::Error
            }
        }

        result
    }

    fn success_message(&self) {
        rule_success_message!("SCM13");
    }

    fn code(&self) -> &'static str {
        "TT13"
    }
}

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_v02 => ("TT13/vardeclaration.js", VarKeywordChecker, "V02", Error),
       test_valid => ("TT13/constdeclaration.js", VarKeywordChecker, "", Ok),
    }
}
