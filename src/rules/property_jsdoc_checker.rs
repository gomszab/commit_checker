use oxc::ast::{AstKind, ast::PropertyDefinition};
use oxc_semantic::{AstNodes, JSDoc, JSDocFinder};
use rust_i18n::t;

use crate::api::{Handler, HandlerResult};

pub struct PropertyJsDocChecker;

impl Handler for PropertyJsDocChecker {
    fn handle<'a>(&self, context: &'a crate::api::FileContext<'a>) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = &context.semantic;
        let nodes = semantic.nodes();

        for (decl, jsdoc) in get_all_property_decl_jsdocs(nodes, semantic.jsdoc()) {
            let decl_start = decl.span.start;
            let Some(jsdoc) = jsdoc else {
                errors.push(
                    t!(
                        "PD01",
                        line = context.get_line(decl_start),
                        property = context.lines[context.get_line(decl_start) - 1],
                        highlight = format_args!(
                            "{}{}",
                            " ".repeat(context.get_column(decl_start) - 1),
                            "^".repeat((decl.span.end - decl_start) as usize)
                        )
                    )
                    .to_string(),
                );
                continue;
            };

            let type_tag = jsdoc.tags().iter().find(|tag| tag.kind.parsed() == "type");
            if type_tag.is_none() {
                errors.push(
                    t!(
                        "PD02",
                        line = context.get_line(decl_start),
                        property = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(decl_start) - 1]
                            .to_vec()
                            .join("\n"),
                        highlight = format_args!(
                            "{}{}",
                            " ".repeat(context.get_column(decl_start) - 1),
                            "^".repeat((decl.span.end - decl_start) as usize)
                        )
                    )
                    .to_string(),
                );
                continue;
            };
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self) -> String {
        t!("SCM07").to_string()
    }
    fn title(&self) -> String {
        t!("TT07").to_string()
    }
}

/// Returns all property declarations along with their jsdocs.
fn get_all_property_decl_jsdocs<'a>(
    nodes: &'a AstNodes,
    jsdoc_finder: &'a JSDocFinder<'a>,
) -> Vec<(&'a PropertyDefinition<'a>, Option<JSDoc<'a>>)> {
    let mut declarations = Vec::new();
    for node in nodes {
        let AstKind::PropertyDefinition(def) = node.kind() else {
            continue;
        };
        declarations.push((def, jsdoc_finder.get_one_by_node(nodes, node)));
    }

    declarations
}
