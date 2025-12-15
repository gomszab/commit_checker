use oxc::ast::{AstKind, ast::BindingPatternKind};
use oxc_semantic::JSDoc;

use crate::api::{Handler, HandlerResult};

pub struct VariableJsDocChecker;

impl Handler for VariableJsDocChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        let nodes = context.semantic.get().unwrap().nodes();

        for definition in nodes.iter() {
            if let AstKind::VariableDeclarator(var) = definition.kind() {
                println!(
                    "{}",
                    &context.program.source_text[var.span.start as usize..var.span.end as usize]
                );
                if let BindingPatternKind::BindingIdentifier(identifier) = &var.id.kind {
                    let name = identifier.name;
                    let start = var.span.start;

                    let jsdoc = context.semantic.get().unwrap().jsdoc();
                    // .get_one_by_node(&nodes, definition);
                    // if let Some(jsdoc) = jsdoc
                    //     && includes_correct_type_tag(&jsdoc)
                    // {
                    //     println!("szia");
                    //     continue;
                    // } else {
                    //     errors.push(format!(
                    //         "sor: {}: Nincs jsdoc dokumentáció\n{}\n{}",
                    //         context.get_line(start),
                    //         context.lines[context.get_line(start) - 1],
                    //         format!(
                    //             "{}{}",
                    //             " ".repeat(context.get_column(start) - 1),
                    //             "^".repeat(name.len())
                    //         )
                    //     ));
                    // };
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
        format!("JsDocok szama rendben")
    }
    fn title(&self) -> String {
        format!("JSDoc szamolas folyamatban")
    }
}

fn includes_correct_type_tag(jsdoc: &JSDoc) -> bool {
    for tag in jsdoc.tags().iter() {
        let (type_part, comment_part) = tag.type_comment();
        if let Some(ty) = type_part
            && ty.parsed().len() > 0
            && comment_part.parsed().len() > 0
        {
            return true;
        }
    }

    false
}
