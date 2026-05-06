use commit_checker_message_handler::{
    info_message, rule_success_message, software_error, validation_error,
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
        let mut result = HandlerResult::Ok;
        let semantic = context.semantic.get().unwrap();
        for node in semantic.nodes() {
            let AstKind::Class(class) = node.kind() else {
                continue;
            };

            let Some(id) = &class.id else {
                // The error is handled only in the ClassNameChecker.
                continue;
            };

            let constructor = &class.body.body.iter().find(|element| {
                matches!(
                    element.method_definition_kind(),
                    Some(MethodDefinitionKind::Constructor)
                )
            });

            let Some(ClassElement::MethodDefinition(constructor)) = constructor else {
                validation_error!(
                    &mut ioc.message_handler,
                    "C04",
                    &context.file_name,
                    context.get_line(class.span.start) as usize,
                    context.get_column(id.span.start) - 1,
                    context.get_column(id.span.end) as usize,
                    class = context.lines[context.get_line(class.span.start) - 1]
                );

                result = HandlerResult::Error;
                continue;
            };

            if let Some(super_class) = &class.super_class
                && let Expression::Identifier(super_id) = super_class
            {
                // I honestly can't be bothered to handle a constructor missing a body.
                // TODO: handle missing constructor body
                let body = constructor.value.body.as_ref();

                if let Some(body) = body
                    && !super_exists(&body.statements)
                {
                    validation_error!(
                        &mut ioc.message_handler,
                        "C05",
                        &context.file_name,
                        context.get_line(class.span.start) as usize,
                        context.get_column(id.span.start) - 1,
                        context.get_column(super_id.span.end) as usize,
                        class = context.lines[context.get_line(class.span.start) - 1]
                    );
                    result = HandlerResult::Error;
                } else if body.is_none() {
                    software_error!(&mut ioc.message_handler, "SW05", None); // never happened becuse of sw04 edit: only in commented codelines
                    result = HandlerResult::Error;
                };
            }
        }

        result
    }

    fn success_message(&self, ioc: &mut CommitCheckerIoC) -> () {
        rule_success_message!(&mut ioc.message_handler, "SCM01");
    }

    fn code(&self) -> &'static str {
        "TT01"
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
#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_c04 => ("TT01/missing_constructor.js", ClassChecker, "C04", Error),
       test_c05 => ("TT01/missing_super.js", ClassChecker, "C05", Error),
       //test_sw05 => ("TT01/no_constructor_body.js", ClassChecker, "SW05", Error),
       test_constructor_exist => ("TT01/constructor_exist.js", ClassChecker, "", Ok),
       test_super_called => ("TT01/super_called.js", ClassChecker, "", Ok),
    }
}
