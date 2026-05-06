use commit_checker_message_handler::{info_message, rule_success_message, validation_error};
use oxc::ast::{AstKind, ast::PropertyDefinition};
use oxc_semantic::{AstNodes, JSDoc, JSDocFinder};

use crate::rules::api::{Handler, HandlerResult};

pub struct PropertyJsDocChecker;

impl Handler for PropertyJsDocChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        let semantic = context.semantic.get().unwrap();
        let nodes = semantic.nodes();

        for (decl, jsdoc) in get_all_property_decl_jsdocs(nodes, semantic.jsdoc()) {
            let decl_start = decl.span.start;
            let Some(jsdoc) = jsdoc else {
                validation_error!(
                    &mut ioc.message_handler,
                    "PD01",
                    &context.file_name,
                    context.get_line(decl_start) as usize,
                    context.get_column(decl_start) - 1,
                    (decl.span.end) as usize,
                    property = context.lines[context.get_line(decl_start) - 1]
                );
                result = HandlerResult::Error;
                continue;
            };

            let type_tag = jsdoc.tags().iter().find(|tag| tag.kind.parsed() == "type");
            if type_tag.is_none() {
                validation_error!(
                    &mut ioc.message_handler,
                    "PD02",
                    &context.file_name,
                    context.get_line(decl_start) as usize,
                    context.get_column(decl_start) - 1,
                    decl.span.end as usize,
                    property = context.lines
                        [context.get_line(jsdoc.span.start) - 1..=context.get_line(decl_start) - 1]
                        .to_vec()
                        .join("\n")
                );
                result = HandlerResult::Error;
                continue;
            };
        }

        result
    }

    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM07");
    }

    fn code(&self) -> &'static str {
        "TT07"
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

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_pd01 => ("TT07/missing_jsdoc.js", PropertyJsDocChecker, "PD01", Error),
       test_pd02 => ("TT07/missing_type.js", PropertyJsDocChecker, "PD02", Error),
       test_valid => ("TT09/valid.js", PropertyJsDocChecker, "", Ok),
    }
}
