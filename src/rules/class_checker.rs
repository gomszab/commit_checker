use oxc::ast::{
    AstKind,
    ast::{ClassElement, Expression, MethodDefinitionKind, Statement},
};

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
                errors.push(format!(
                    "sor: {}: Az osztálynak nincs neve",
                    context.get_line(class.span.start)
                ));
                continue;
            };

            let constructor = &class.body.body.iter().find(|element| {
                matches!(
                    element.method_definition_kind(),
                    Some(MethodDefinitionKind::Constructor)
                )
            });

            let Some(ClassElement::MethodDefinition(constructor)) = constructor else {
                errors.push(format!(
                    "sor: {}: Az osztálynak nincs konstruktora\n{}\n{}",
                    context.get_line(class.span.start),
                    context.lines[context.get_line(class.span.start) - 1],
                    format_args!(
                        "{}{}",
                        " ".repeat(context.get_column(id.span.start) - 1),
                        "^".repeat((class.body.span.start - 1 - id.span.start) as usize)
                    )
                ));
                continue;
            };

            if let Some(super_class) = &class.super_class
                && let Expression::Identifier(super_id) = super_class
            {
                // I honestly can't be bothered to handle a constructor missing a body.
                let body = constructor
                    .value
                    .body
                    .as_ref()
                    .expect("A konstruktornak nincs body-ja");

                if !super_exists(&body.statements) {
                    errors.push(format!(
                        "sor: {}: Az osztály leszármazik egy másik osztályból, de nem hívod meg a super()-t a konstruktorban\n{}\n{}",
                        context.get_line(class.span.start),
                        context.lines[context.get_line(class.span.start) - 1],
                        format_args!(
                            "{}{}",
                            " ".repeat(context.get_column(id.span.start) - 1),
                            "^".repeat((super_id.span.end - id.span.start) as usize)
                        )
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
        format!("Osztálydefiníciók rendben")
    }

    fn title(&self) -> String {
        format!("Osztálydefiníciók ellenőrzése...")
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
