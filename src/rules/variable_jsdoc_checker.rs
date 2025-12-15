use oxc::{
    ast::{AstKind, ast::VariableDeclarator},
    span::ContentEq,
};
use oxc_semantic::{AstNodes, JSDoc, JSDocFinder};

use crate::api::{Handler, HandlerResult};

pub struct VariableJsDocChecker;

impl Handler for VariableJsDocChecker {
    fn handle<'a>(&self, context: &'a crate::api::FileContext<'a>) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        let nodes = semantic.nodes();

        // Needed, because the first declaration's jsdoc is attached to the VariableDeclaration,
        // not the VariableDeclarator, so it would erroneously say that it does not have a jsdoc.
        for (decl, jsdoc) in get_all_var_decl_jsdocs(nodes, semantic.jsdoc()) {
            let decl_start = decl.span.start;
            let Some(jsdoc) = jsdoc else {
                errors.push(format!(
                    "sor: {}: A változónak nincs JSDoc-ja\n{}\n{}",
                    context.get_line(decl_start),
                    context.lines[context.get_line(decl_start) - 1],
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(decl_start) - 1),
                        "^".repeat((decl.span.end - decl_start) as usize)
                    )
                ));
                continue;
            };

            let type_tag = jsdoc.tags().iter().find(|tag| tag.kind.parsed() == "type");
            let Some(tag) = type_tag else {
                errors.push(format!(
                    "sor: {}: A változó JSDoc-jában nincsen @type\n{}\n{}",
                    context.get_line(decl_start),
                    context.lines
                        [context.get_line(jsdoc.span.start) - 1..=context.get_line(decl_start) - 1]
                        .to_vec()
                        .join("\n"),
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(decl_start) - 1),
                        "^".repeat((decl.span.end - decl_start) as usize)
                    )
                ));
                continue;
            };

            let type_comment = tag.type_comment();
            if let None = type_comment.0
                && type_comment.1.parsed().len() == 0
            {
                errors.push(format!(
                    "sor: {}: A @type JSDoc-nak nincs se típus, se leírás megadva\n{}\n{}",
                    context.get_line(tag.span.start),
                    context.lines
                        [context.get_line(jsdoc.span.start) - 1..=context.get_line(decl_start) - 1]
                        .to_vec()
                        .join("\n"),
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(decl_start) - 1),
                        "^".repeat((decl.span.end - decl_start) as usize)
                    )
                ));
                continue;
            } else if type_comment.1.parsed().len() == 0 {
                errors.push(format!(
                    "sor: {}: A @type JSDoc-nak nincs leírás megadva\n{}\n{}",
                    context.get_line(tag.span.start),
                    context.lines
                        [context.get_line(jsdoc.span.start) - 1..=context.get_line(decl_start) - 1]
                        .to_vec()
                        .join("\n"),
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(decl_start) - 1),
                        "^".repeat((decl.span.end - decl_start) as usize)
                    )
                ));
                continue;
            } else if let None = type_comment.0 {
                errors.push(format!(
                    "sor: {}: A @type JSDoc-nak nincs típus megadva\n{}\n{}",
                    context.get_line(tag.span.start),
                    context.lines
                        [context.get_line(jsdoc.span.start) - 1..=context.get_line(decl_start) - 1]
                        .to_vec()
                        .join("\n"),
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(decl_start) - 1),
                        "^".repeat((decl.span.end - decl_start) as usize)
                    )
                ));
                continue;
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self) -> String {
        format!("Változók JSDocjai rendben")
    }
    fn title(&self) -> String {
        format!("Változók JSDocjainak ellenőrzése...")
    }
}

/// Returns all variable declarations along with their
fn get_all_var_decl_jsdocs<'a>(
    nodes: &'a AstNodes,
    jsdoc_finder: &'a JSDocFinder<'a>,
) -> Vec<(&'a VariableDeclarator<'a>, Option<JSDoc<'a>>)> {
    let mut declarations = Vec::new();
    for node in nodes {
        // Needed, because the first VariableDeclarator has its jsdoc attached to its
        // VariableDeclaration, not the VariableDeclarator.
        if let AstKind::VariableDeclaration(decl) = node.kind() {
            declarations.push((
                &decl.declarations[0],
                jsdoc_finder.get_one_by_node(nodes, node),
            ));
        } else if let AstKind::VariableDeclarator(decl) = node.kind()
            && !declarations.iter().any(|node| decl.content_eq(node.0))
        // ^^ Filter out declarations
        // that we already processed.
        {
            declarations.push((&decl, jsdoc_finder.get_one_by_node(nodes, node)));
        }
    }

    declarations
}
