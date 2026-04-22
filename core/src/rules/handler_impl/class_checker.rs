use commit_checker_message_handler::{
    MessageHandlerApi,info_message, rule_success_message, software_error, validation_error
};
use oxc::ast::{
    AstKind,
    ast::{ClassElement, Expression, MethodDefinitionKind, Statement},
};

use crate::{
    api::CommitCheckerIoC,
    rules::api::{Handler, HandlerResult},
};

pub struct ClassChecker;

impl Handler for ClassChecker {
    fn handle(
        &self,
        context: &crate::rules::api::FileContext,
        ioc: &mut CommitCheckerIoC,
    ) -> HandlerResult {
        let mut fail = false;
        let semantic = context.semantic.get().unwrap();
        for node in semantic.nodes() {
            let AstKind::Class(class) = node.kind() else {
                continue;
            };

            let Some(id) = &class.id else {
                 validation_error!(&mut ioc.message_handler,"C01", &context.file_name, context.get_line(class.span.start) as usize, 0_usize, 1_usize, nope="");
                fail = true;
                continue;
            };

            let constructor = &class.body.body.iter().find(|element| {
                matches!(
                    element.method_definition_kind(),
                    Some(MethodDefinitionKind::Constructor)
                )
            });

            let Some(ClassElement::MethodDefinition(constructor)) = constructor else {
                validation_error!(&mut ioc.message_handler,"C04", &context.file_name, context.get_line(class.span.start) as usize, context.get_column(id.span.start) - 1, (class.body.span.start - 1) as usize, class = context.lines[context.get_line(class.span.start) - 1] );

                fail = true;
                continue;
            };

            if let Some(super_class) = &class.super_class
                && let Expression::Identifier(super_id) = super_class
            {
                // I honestly can't be bothered to handle a constructor missing a body.
                // TODO: handle missing constructor body
                let body = constructor.value.body.as_ref();
                // .expect(&t!("SW05").to_string());
                if body.is_none() {
                    software_error!(&mut ioc.message_handler, "SW05", None);
                }
                let body = body.unwrap();

                if !super_exists(&body.statements) {
                    validation_error!(&mut ioc.message_handler,"C05", &context.file_name, context.get_line(class.span.start) as usize, context.get_column(id.span.start) - 1, super_id.span.end as usize, class = context.lines[context.get_line(class.span.start) - 1] );
                    fail = false;
                }
            }
        }

        if fail {
            HandlerResult::Error(vec![])
        } else {
            HandlerResult::Ok
        }
    }

    fn success_message(&self, ioc: &mut CommitCheckerIoC) -> () {
        rule_success_message!(&mut ioc.message_handler, "SCM01");
    }

    fn title(&self, ioc: &mut CommitCheckerIoC) -> () {
        info_message!(&mut ioc.message_handler, "TT01");
    }
}

fn super_exists(stmts: &oxc::allocator::Vec<Statement>) -> bool {
    for s in stmts {
        if let Statement::ExpressionStatement(stmt) = s
            && let Expression::CallExpression(call_expr) = &stmt.expression
            && let Expression::Super(_) = call_expr.callee
        {
            return true;
        }
    }

    false
}
