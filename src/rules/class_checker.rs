use oxc::ast::{
    AstKind,
    ast::{ClassElement, Expression, MethodDefinitionKind, Statement},
};
use rust_i18n::t;

use crate::api::{Handler, HandlerResult};

pub struct ClassChecker;

impl Handler for ClassChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        for node in semantic.nodes() {
            let AstKind::Class(class) = node.kind() else {
                continue;
            };

            let Some(id) = &class.id else {
                errors.push(t!("C01", line = context.get_line(class.span.start)).to_string());
                continue;
            };

            let constructor = &class.body.body.iter().find(|element| {
                matches!(
                    element.method_definition_kind(),
                    Some(MethodDefinitionKind::Constructor)
                )
            });

            let Some(ClassElement::MethodDefinition(constructor)) = constructor else {
                errors.push(
                    t!(
                        "C04",
                        line = context.get_line(class.span.start),
                        class = context.lines[context.get_line(class.span.start) - 1],
                        highlight = format_args!(
                            "{}{}",
                            " ".repeat(context.get_column(id.span.start) - 1),
                            "^".repeat((class.body.span.start - 1 - id.span.start) as usize)
                        )
                    )
                    .to_string(),
                );
                continue;
            };

            if let Some(super_class) = &class.super_class
                && let Expression::Identifier(super_id) = super_class
            {
                // I honestly can't be bothered to handle a constructor missing a body.
                // TODO: handle missing constructor body
                let body = constructor
                    .value
                    .body
                    .as_ref()
                    .expect(&t!("SW05").to_string());

                if !super_exists(&body.statements) {
                    errors.push(
                        t!(
                            "C05",
                            line = context.get_line(class.span.start),
                            class = context.lines[context.get_line(class.span.start) - 1],
                            highlight = format_args!(
                                "{}{}",
                                " ".repeat(context.get_column(id.span.start) - 1),
                                "^".repeat((super_id.span.end - id.span.start) as usize)
                            )
                        )
                        .to_string(),
                    );
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
        t!("SCM01").to_string()
    }

    fn title(&self) -> String {
        t!("TT01").to_string()
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
