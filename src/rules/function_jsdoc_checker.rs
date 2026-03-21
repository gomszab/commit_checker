use oxc::ast::{
    AstKind,
    ast::{Function, FunctionType},
};
use oxc_semantic::{AstNodes, JSDoc, JSDocFinder, JSDocTag};
use rust_i18n::t;

use crate::api::{Handler, HandlerResult};

pub struct FunctionJsDocChecker;

impl Handler for FunctionJsDocChecker {
    fn handle<'a>(&self, context: &'a crate::api::FileContext<'a>) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        let nodes = semantic.nodes();

        for (start, decl, jsdoc) in get_all_func_decl_jsdocs(nodes, semantic.jsdoc()) {
            let decl_start = start;
            let Some(body) = &decl.body else {
                panic!("{}", t!("SW06").to_string());
            };

            let Some(jsdoc) = jsdoc else {
                errors.push(
                    t!(
                        "FD01",
                        line = context.get_line(decl_start),
                        function = context.lines[context.get_line(decl_start) - 1],
                        highlight = format!(
                            "{}{}",
                            " ".repeat(context.get_column(decl_start) - 1),
                            "^".repeat((body.span.start - 1 - decl_start) as usize)
                        )
                    )
                    .to_string(),
                );
                continue;
            };

            if jsdoc.comment().parsed().len() == 0 {
                errors.push(
                    t!(
                        "FD02",
                        line = context.get_line(decl_start),
                        function = context.lines[context.get_line(decl_start) - 1],
                        highlight = format!(
                            "{}{}",
                            " ".repeat(context.get_column(decl_start) - 1),
                            "^".repeat((body.span.start - 1 - decl_start) as usize)
                        )
                    )
                    .to_string(),
                );
            }

            let param_tags = jsdoc
                .tags()
                .iter()
                .filter(|tag| tag.kind.parsed() == "param")
                .collect::<Vec<&JSDocTag>>();
            if param_tags.len() != decl.params.parameters_count() {
                errors.push(
                    t!(
                        "FD03",
                        line = context.get_line(decl_start),
                        function = context.lines[context.get_line(decl_start) - 1],
                        highlight = format!(
                            "{}{}",
                            " ".repeat(context.get_column(decl_start) - 1),
                            "^".repeat((body.span.start - 1 - decl_start) as usize)
                        )
                    )
                    .to_string(),
                );
            }

            let mut params = decl
                .params
                .iter_bindings()
                .filter_map(|ident| ident.get_identifier_name());
            for tag in param_tags {
                let (type_part, name_part, comment_part) = tag.type_name_comment();

                if let None = type_part {
                    errors.push(
                        t!(
                            "FD04",
                            line = context.get_line(tag.span.start),
                            jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                                ..=context.get_line(jsdoc.span.end) - 1]
                                .to_vec()
                                .join("\n"),
                            highlight = "^"
                                .repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                        )
                        .to_string(),
                    );
                    continue;
                };

                if name_part.is_none()
                    || (name_part.is_some() && name_part.unwrap().parsed() == "*")
                {
                    errors.push(
                        t!(
                            "FD05",
                            line = context.get_line(tag.span.start),
                            jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                                ..=context.get_line(jsdoc.span.end) - 1]
                                .to_vec()
                                .join("\n"),
                            highlight = "^"
                                .repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                        )
                        .to_string(),
                    );
                } else {
                    // If there is no name, then we skip checking if it is in the parameter list.
                    // We can unwrap because we already checked if it is none.
                    if !params.any(|param| param == name_part.unwrap().parsed()) {
                        errors.push(
                            t!(
                                "FD06",
                                line = context.get_line(tag.span.start),
                                jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                                    ..=context.get_line(jsdoc.span.end) - 1]
                                    .to_vec()
                                    .join("\n"),
                                highlight = "^".repeat(
                                    context.lines[context.get_line(jsdoc.span.end - 2)].len()
                                )
                            )
                            .to_string(),
                        );
                    }
                }

                if comment_part.parsed().len() == 0 {
                    errors.push(
                        t!(
                            "FD07",
                            line = context.get_line(tag.span.start),
                            jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                                ..=context.get_line(jsdoc.span.end) - 1]
                                .to_vec()
                                .join("\n"),
                            highlight = "^"
                                .repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                        )
                        .to_string(),
                    );
                }
            }

            let returns_tag = jsdoc
                .tags()
                .iter()
                .find(|tag| tag.kind.parsed() == "returns");
            let Some(returns_tag) = returns_tag else {
                errors.push(
                    t!(
                        "FD08",
                        line = context.get_line(decl_start),
                        function = context.lines[context.get_line(decl_start) - 1],
                        highlight = format!(
                            "{}{}",
                            " ".repeat(context.get_column(decl_start) - 1),
                            "^".repeat((body.span.start - 1 - decl_start) as usize)
                        )
                    )
                    .to_string(),
                );
                continue;
            };

            let type_part = returns_tag.r#type();
            if let None = type_part {
                errors.push(
                    t!(
                        "FD09",
                        line = context.get_line(returns_tag.span.start),
                        jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                        highlight =
                            "^".repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                    )
                    .to_string(),
                );
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self) -> String {
        t!("SCM04").to_string()
    }
    fn title(&self) -> String {
        t!("TT04").to_string()
    }
}

/// Returns all function declarations along with their jsdocs and their span starts.
/// We return the start of the span too, because method definitions are different and the
/// function's span only covers the "()".
fn get_all_func_decl_jsdocs<'a>(
    nodes: &'a AstNodes,
    jsdoc_finder: &'a JSDocFinder<'a>,
) -> Vec<(u32, &'a Function<'a>, Option<JSDoc<'a>>)> {
    let mut declarations = Vec::new();
    for node in nodes {
        if let AstKind::Function(decl) = node.kind()
            && let FunctionType::FunctionDeclaration = decl.r#type
        {
            declarations.push((
                decl.span.start,
                decl,
                jsdoc_finder.get_one_by_node(nodes, node),
            ));
        } else if let AstKind::MethodDefinition(def) = node.kind() {
            declarations.push((
                def.span.start,
                &def.value,
                jsdoc_finder.get_one_by_node(nodes, node),
            ));
        }
    }

    declarations
}
