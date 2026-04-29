use commit_checker_message_handler::{info_message, rule_success_message, validation_error};
use oxc::ast::ast::{Statement, VariableDeclarationKind};

use crate::rules::api::{Handler, HandlerResult};

pub struct VarKeywordChecker;

impl Handler for VarKeywordChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        for declaration in context.program.body.iter() {
            if let Statement::VariableDeclaration(decl) = declaration
                && let VariableDeclarationKind::Var = decl.kind
            {
                validation_error!(
                    &mut ioc.message_handler,
                    "V02",
                    &context.file_name,
                    context.get_line(decl.span.start) as usize,
                    0_usize,
                    1_usize as usize
                );
                result = HandlerResult::Error
            }
        }

        result
    }

    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM13");
    }

    fn title(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        info_message!(&mut ioc.message_handler, "TT13");
    }
}
