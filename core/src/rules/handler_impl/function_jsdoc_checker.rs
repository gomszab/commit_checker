use commit_checker_message_handler::{rule_success_message, software_error, validation_error};
use oxc::ast::{
    AstKind,
    ast::{Function, FunctionType},
};
use oxc_semantic::{AstNodes, JSDoc, JSDocFinder, JSDocTag};

use crate::rules::api::{Handler, HandlerResult};

pub struct FunctionJsDocChecker;

impl Handler for FunctionJsDocChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        let semantic = context.semantic.get().unwrap();
        let nodes = semantic.nodes();

        for (start, decl, jsdoc) in get_all_func_decl_jsdocs(nodes, semantic.jsdoc()) {
            let decl_start = start;
            let Some(body) = &decl.body else {
                software_error!("SW06"); //FIXME: no testcase
                result = HandlerResult::Error;
                continue;
            };

            let Some(jsdoc) = jsdoc else {
                validation_error!(
                    "FD01",
                    context.get_line(decl_start) as usize,
                    context.get_column(decl_start) - 1,
                    (body.span.start - 1 - decl_start) as usize,
                    function = context.lines[context.get_line(decl_start) - 1]
                );
                result = HandlerResult::Error;
                continue;
            };

            if jsdoc.comment().parsed().len() == 0 {
                validation_error!(
                    "FD02",
                    context.get_line(decl_start) as usize,
                    context.get_column(decl_start) - 1,
                    (body.span.start - 1 - decl_start) as usize,
                    function = context.lines[context.get_line(decl_start) - 1]
                );
                result = HandlerResult::Error;
            }

            let param_tags = jsdoc
                .tags()
                .iter()
                .filter(|tag| tag.kind.parsed() == "param")
                .collect::<Vec<&JSDocTag>>();
            if param_tags.len() != decl.params.parameters_count() {
                validation_error!(
                    "FD03",
                    context.get_line(decl_start) as usize,
                    context.get_column(decl_start) - 1,
                    (body.span.start - 1 - decl_start) as usize,
                    function = context.lines[context.get_line(decl_start) - 1]
                );
                result = HandlerResult::Error;
            }

            let mut params = decl
                .params
                .iter_bindings()
                .filter_map(|ident| ident.get_identifier_name());
            for tag in param_tags {
                let (type_part, name_part, comment_part) = tag.type_name_comment();

                if let None = type_part {
                    validation_error!(
                        "FD04",
                        context.get_line(tag.span.start),
                        0_usize,
                        context.get_line(tag.span.start)
                            + context.lines[context.get_line(tag.span.start)].len(),
                        jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n")
                    );
                    result = HandlerResult::Error;
                    continue;
                };

                if name_part.is_none()
                    || (name_part.is_some() && name_part.unwrap().parsed() == "*")
                {
                    validation_error!(
                        "FD05",
                        context.get_line(tag.span.start),
                        0_usize,
                        context.get_line(tag.span.start)
                            + context.lines[context.get_line(tag.span.start)].len(),
                        jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n")
                    );
                    result = HandlerResult::Error;
                    continue;
                } else {
                    // If there is no name, then we skip checking if it is in the parameter list.
                    // We can unwrap because we already checked if it is none.
                    if !params.any(|param| param == name_part.unwrap().parsed()) {
                        validation_error!(
                            "FD06",
                            context.get_line(tag.span.start),
                            0_usize,
                            context.get_line(tag.span.start)
                                + context.lines[context.get_line(tag.span.start)].len(),
                            jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                                ..=context.get_line(jsdoc.span.end) - 1]
                                .to_vec()
                                .join("\n")
                        );
                        result = HandlerResult::Error;
                    }
                }

                if comment_part.parsed().len() == 0 {
                    validation_error!(
                        "FD07",
                        context.get_line(tag.span.start),
                        0_usize,
                        context.get_line(tag.span.start)
                            + context.lines[context.get_line(tag.span.start)].len(),
                        jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n")
                    );
                    result = HandlerResult::Error;
                }
            }

            let returns_tag = jsdoc
                .tags()
                .iter()
                .find(|tag| tag.kind.parsed() == "returns");
            let Some(returns_tag) = returns_tag else {
                validation_error!(
                    "FD08",
                    context.get_line(decl_start),
                    context.get_column(decl_start) - 1 as usize,
                    (body.span.start - 1 - decl_start) as usize,
                    function = context.lines[context.get_line(decl_start) - 1]
                );
                result = HandlerResult::Error;
                continue;
            };

            let type_part = returns_tag.r#type();
            if let None = type_part {
                validation_error!(
                    "FD09",
                    context.get_line(returns_tag.span.start),
                    0_usize,
                    context.get_line(returns_tag.span.start)
                        + context.lines[context.get_line(returns_tag.span.start)].len(),
                    jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                        ..=context.get_line(jsdoc.span.end) - 1]
                        .to_vec()
                        .join("\n"),
                );
                result = HandlerResult::Error;
            }
        }

        result
    }

    fn success_message(&self) {
        rule_success_message!("SCM04");
    }

    fn code(&self) -> &'static str {
        "TT04"
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

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_fd01 => ("TT04/no_jsdoc.js", FunctionJsDocChecker, "FD01", Error),
       test_fd02 => ("TT04/no_description.js", FunctionJsDocChecker, "FD02", Error),
       test_fd03_func => ("TT04/invalid_param_func.js", FunctionJsDocChecker, "FD03", Error),
       test_fd03_jsdoc => ("TT04/invalid_param_jsdoc.js", FunctionJsDocChecker, "FD03", Error),
       test_fd04 => ("TT04/no_param_type.js", FunctionJsDocChecker, "FD04", Error),
       test_fd05 => ("TT04/no_param_name.js", FunctionJsDocChecker, "FD05", Error),
       test_fd06 => ("TT04/not_exist_param_jsdoc.js", FunctionJsDocChecker, "FD06", Error),
       test_fd07 => ("TT04/no_param_desc.js", FunctionJsDocChecker, "FD07", Error),
       test_fd08 => ("TT04/no_return.js", FunctionJsDocChecker, "FD08", Error),
       test_fd09 => ("TT04/no_returntype.js", FunctionJsDocChecker, "FD09", Error),
       test_valid => ("TT04/valid.js", FunctionJsDocChecker, "", Ok),
    }
}
