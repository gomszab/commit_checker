use commit_checker_message_handler::{rule_success_message, validation_error};
use oxc::{
    ast::{AstKind, ast::VariableDeclarator},
    span::ContentEq,
};
use oxc_semantic::{AstNodes, JSDoc, JSDocFinder};

use crate::rules::api::{Handler, HandlerResult};

pub struct VariableJsDocChecker;

impl Handler for VariableJsDocChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        let semantic = context.semantic.get().unwrap();
        let nodes = semantic.nodes();

        for (decl, jsdoc) in get_all_var_decl_jsdocs(nodes, semantic.jsdoc()) {
            let decl_start = decl.span.start;
            let Some(jsdoc) = jsdoc else {
                validation_error!(
                    &mut ioc.message_handler,
                    "VD01",
                    &context.file_name,
                    context.get_line(decl_start) as usize,
                    context.get_column(decl_start) - 1,
                    decl.span.end as usize,
                    variable = context.lines[context.get_line(decl_start) - 1],
                );
                result = HandlerResult::Error;
                continue;
            };

            let type_tag = jsdoc.tags().iter().find(|tag| tag.kind.parsed() == "type");
            if type_tag.is_none() {
                validation_error!(
                    &mut ioc.message_handler,
                    "VD02",
                    &context.file_name,
                    context.get_line(decl_start) as usize,
                    context.get_column(decl_start) - 1,
                    decl.span.end as usize,
                    variable = context.lines
                        [context.get_line(jsdoc.span.start) - 1..=context.get_line(decl_start) - 1]
                        .to_vec()
                        .join("\n"),
                );
                result = HandlerResult::Error;
                continue;
            };
        }

        result
    }

    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM14");
    }

    fn code(&self) -> &'static str {
        "TT14"
    }
}

/// Returns all variable declarations along with their jsdocs, except those initialized in for
/// loops.
fn get_all_var_decl_jsdocs<'a>(
    nodes: &'a AstNodes,
    jsdoc_finder: &'a JSDocFinder<'a>,
) -> Vec<(&'a VariableDeclarator<'a>, Option<JSDoc<'a>>)> {
    let mut declarations = Vec::new();
    for node in nodes {
        // Needed, because the first VariableDeclarator has its jsdoc attached to its
        // VariableDeclaration, not the VariableDeclarator.
        if let AstKind::VariableDeclaration(decl) = node.kind() {
            // We do not need jsdocs for for loop variables.
            if matches!(
                nodes.parent_kind(node.id()),
                AstKind::ForStatement(_) | AstKind::ForOfStatement(_) | AstKind::ForInStatement(_)
            ) {
                continue;
            }
            declarations.push((
                &decl.declarations[0],
                jsdoc_finder.get_one_by_node(nodes, node),
            ));
        } else if let AstKind::VariableDeclarator(decl) = node.kind()
            && !declarations.iter().any(|node| decl.content_eq(node.0))
        // ^^ Filter out declarations
        // that we already processed.
        {
            // We do not need jsdocs for for loop variables. The for loop will be a parent of the
            // parent of this node.
            if matches!(
                nodes.parent_kind(nodes.parent_id(node.id())),
                AstKind::ForStatement(_) | AstKind::ForOfStatement(_) | AstKind::ForInStatement(_)
            ) {
                continue;
            }
            declarations.push((&decl, jsdoc_finder.get_one_by_node(nodes, node)));
        }
    }

    declarations
}

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_vd01 => ("TT14/nojsdoc.js", VariableJsDocChecker, "VD01", Error),
       test_vd02 => ("TT14/jsdoc_notype.js", VariableJsDocChecker, "VD02", Error),
       test_valid => ("TT14/valid.js", VariableJsDocChecker, "", Ok),
    }
}
