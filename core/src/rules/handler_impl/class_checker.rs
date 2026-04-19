use commit_checker_message_handler::{MessageHandlerApi, error_message, rule_break_message, success_message, title_message};
use oxc::ast::{
    AstKind,
    ast::{ClassElement, Expression, MethodDefinitionKind, Statement},
};

use crate::{api::CommitCheckerIoC, rules::api::{Handler, HandlerResult}};

pub struct ClassChecker;

impl Handler for ClassChecker {
    fn handle(&self, context: &crate::rules::api::FileContext, ioc: &mut CommitCheckerIoC) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        for node in semantic.nodes() {
            let AstKind::Class(class) = node.kind() else {
                continue;
            }; 

            let Some(id) = &class.id else {
                rule_break_message!(&mut ioc.message_handler, "C01", &context.file_name, line = context.get_line(class.span.start));
                continue;
            };

            let constructor = &class.body.body.iter().find(|element| {
                matches!(
                    element.method_definition_kind(),
                    Some(MethodDefinitionKind::Constructor)
                )
            });

            let Some(ClassElement::MethodDefinition(constructor)) = constructor else {
                // let highlight = format_args!("{}{}",
                //             " ".repeat(context.get_column(id.span.start) - 1),
                //             "^".repeat((class.body.span.start - 1 - id.span.start) as usize)).to_string();
                // let class: String = context.lines[context.get_line(class.span.start) - 1];
                // let line: String = context.get_line(class.span.start);
                // let vec= vec![
                //     ("line".to_string(), line),
                //     ("class".to_string(), class)
                //     ("highlight".to_string(), highlight)];
                // ioc.message_handler.put_message(
                //     commit_checker_message_handler::Message::BreakingRule { code: "C04", 
                //     file_name: context.file_name.to_string(),
                //     params: vec });
                    // rule_break_message!(&ioc.message_handler, "C04", );
               
                    rule_break_message!(
                        &mut ioc.message_handler,
                        "C04",
                        &context.file_name,
                        line = context.get_line(class.span.start),
                        class = context.lines[context.get_line(class.span.start) - 1],
                        highlight = format_args!(
                            "{}{}",
                            " ".repeat(context.get_column(id.span.start) - 1),
                            "^".repeat((class.body.span.start - 1 - id.span.start) as usize)
                        )
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
                    .as_ref();
                    // .expect(&t!("SW05").to_string());
                if body.is_none() {
                    error_message!(&mut ioc.message_handler, "SW05" );
                }
                let body = body.unwrap();

                if !super_exists(&body.statements) {
                    rule_break_message!(
                            &mut ioc.message_handler,
                            "C05",
                            &context.file_name,
                            line = context.get_line(class.span.start),
                            class = context.lines[context.get_line(class.span.start) - 1],
                            highlight = format_args!(
                                "{}{}",
                                " ".repeat(context.get_column(id.span.start) - 1),
                                "^".repeat((super_id.span.end - id.span.start) as usize)
                            )
                        )
                }
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self, ioc: &mut CommitCheckerIoC) -> () {
        success_message!(&mut ioc.message_handler, "SCM01");
    }

    fn title(&self, ioc: &mut CommitCheckerIoC) -> () {
        title_message!(&mut ioc.message_handler, "TT01");
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
