use oxc::ast::{
    AstKind,
    ast::{Function, FunctionType},
};
use oxc_semantic::{AstNodes, JSDoc, JSDocFinder, JSDocTag};

use crate::api::{Handler, HandlerResult};

pub struct FunctionJsDocChecker;

impl Handler for FunctionJsDocChecker {
    fn handle<'a>(&self, context: &'a crate::api::FileContext<'a>) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        let nodes = semantic.nodes();

        // Needed, because the first declaration's jsdoc is attached to the VariableDeclaration,
        // not the VariableDeclarator, so it would erroneously say that it does not have a jsdoc.
        for (decl, jsdoc) in get_all_func_decl_jsdocs(nodes, semantic.jsdoc()) {
            let decl_start = decl.span.start;
            let Some(body) = &decl.body else {
                panic!("Typescriptet nem támogatunk");
            };

            let Some(jsdoc) = jsdoc else {
                errors.push(format!(
                    "sor: {}: A függvénynek nincs JSDoc-ja\n{}\n{}",
                    context.get_line(decl_start),
                    context.lines[context.get_line(decl_start) - 1],
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(decl_start) - 1),
                        "^".repeat((body.span.start - 1 - decl_start) as usize)
                    )
                ));
                continue;
            };

            if jsdoc.comment().parsed().len() == 0 {
                errors.push(format!(
                    "sor: {}: A függvénynek nincs leírása a JSDoc-ban\n{}\n{}",
                    context.get_line(decl_start),
                    context.lines[context.get_line(decl_start) - 1],
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(decl_start) - 1),
                        "^".repeat((body.span.start - 1 - decl_start) as usize)
                    )
                ));
            }

            let param_tags = jsdoc
                .tags()
                .iter()
                .filter(|tag| tag.kind.parsed() == "param")
                .collect::<Vec<&JSDocTag>>();
            if param_tags.len() != decl.params.parameters_count() {
                errors.push(format!(
                    "sor: {}: A függvény paramétereinek száma nem egyezik meg a JSDoc-ban lévő paraméterek számával\n{}\n{}",
                    context.get_line(decl_start),
                    context.lines[context.get_line(decl_start) - 1],
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(decl_start) - 1),
                        "^".repeat((body.span.start - 1 - decl_start) as usize)
                    )
                ));
            }

            let mut params = decl
                .params
                .iter_bindings()
                .filter_map(|ident| ident.get_identifier_name());
            for tag in param_tags {
                let (type_part, name_part, comment_part) = tag.type_name_comment();

                if let None = type_part {
                    errors.push(format!(
                        "sor: {}: A @param-nak nincs típus megadva\n{}\n{}",
                        context.get_line(tag.span.start),
                        context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                        "^".repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                    ));
                    continue;
                };

                if name_part.is_none()
                    || (name_part.is_some() && name_part.unwrap().parsed() == "*")
                {
                    errors.push(format!(
                        "sor: {}: A @param-nak nincs név megadva\n{}\n{}",
                        context.get_line(tag.span.start),
                        context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                        "^".repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                    ));
                } else {
                    // If there is no name, then we skip checking if it is in the parameter list.
                    // We can unwrap because we already checked if it is none.
                    if !params.any(|param| param == name_part.unwrap().parsed()) {
                        errors.push(format!(
                        "sor: {}: A JSDoc olyan paramétert tartalmaz, ami nincs a függvény szignatúrájában\n{}\n{}",
                        context.get_line(tag.span.start),
                        context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                        "^".repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                    ));
                    }
                }

                if comment_part.parsed().len() == 0 {
                    errors.push(format!(
                        "sor: {}: A @param-nak nincs leírás megadva\n{}\n{}",
                        context.get_line(tag.span.start),
                        context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                        "^".repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                    ));
                }
            }

            let returns_tag = jsdoc
                .tags()
                .iter()
                .find(|tag| tag.kind.parsed() == "returns");
            let Some(returns_tag) = returns_tag else {
                errors.push(format!(
                    "sor: {}: A függvény JSDoc-jában nincsen @returns\n{}\n{}",
                    context.get_line(decl_start),
                    context.lines[context.get_line(decl_start) - 1],
                    format!(
                        "{}{}",
                        " ".repeat(context.get_column(decl_start) - 1),
                        "^".repeat((body.span.start - 1 - decl_start) as usize)
                    )
                ));
                continue;
            };

            let type_part = returns_tag.r#type();
            if let None = type_part {
                errors.push(format!(
                    "sor: {}: A @returns-nek nincs típus megadva\n{}\n{}",
                    context.get_line(returns_tag.span.start),
                    context.lines[context.get_line(jsdoc.span.start) - 1
                        ..=context.get_line(jsdoc.span.end) - 1]
                        .to_vec()
                        .join("\n"),
                    "^".repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                ));
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self) -> String {
        format!("Függvények JSDocjai rendben")
    }
    fn title(&self) -> String {
        format!("Függvények JSDocjainak ellenőrzése...")
    }
}

/// Returns all function declarations along with their jsdocs.
fn get_all_func_decl_jsdocs<'a>(
    nodes: &'a AstNodes,
    jsdoc_finder: &'a JSDocFinder<'a>,
) -> Vec<(&'a Function<'a>, Option<JSDoc<'a>>)> {
    let mut declarations = Vec::new();
    for node in nodes {
        if let AstKind::Function(decl) = node.kind()
            && let FunctionType::FunctionDeclaration = decl.r#type
        {
            declarations.push((decl, jsdoc_finder.get_one_by_node(nodes, node)));
        }
    }

    declarations
}
