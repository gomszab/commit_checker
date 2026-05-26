use commit_checker_message_handler::{rule_success_message, validation_error};
use oxc::ast::{AstKind, ast::PropertyKey};

use crate::{
    handler_impl::check_helper::contains_number_or_hungarian_letter,
    rules::api::{Handler, HandlerResult},
};

pub struct PropertyNameChecker;

impl Handler for PropertyNameChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        let semantic = context.semantic.get().unwrap();
        let nodes = semantic.nodes();
        for node in nodes {
            let AstKind::PropertyDefinition(def) = node.kind() else {
                continue;
            };

            let (name, start) = match &def.key {
                PropertyKey::StaticIdentifier(id_name) => {
                    (id_name.name.as_str(), id_name.span.start)
                }
                PropertyKey::PrivateIdentifier(id_name) => {
                    (&id_name.name.as_str()[0..], id_name.span.start + 1)
                }
                _ => continue,
            };

            if name.len() < 5 {
                validation_error!(
                    "P01",
                    context.get_line(start) as usize,
                    context.get_column(start) - 1,
                    (context.get_column(start) + name.len()) as usize,
                    property = context.lines[context.get_line(start) - 1],
                );
                result = HandlerResult::Error;
            }

            if contains_number_or_hungarian_letter(name) {
                validation_error!(
                    "P02",
                    context.get_line(start) as usize,
                    context.get_column(start) - 1,
                    (context.get_column(start) + name.len()) as usize,
                    property = context.lines[context.get_line(start) - 1],
                );
                result = HandlerResult::Error;
            }
        }

        result
    }

    fn success_message(&self) {
        rule_success_message!("SCM08");
    }

    fn code(&self) -> &'static str {
        "TT08"
    }
}

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_p01 => ("TT08/small_prop_name.js", PropertyNameChecker, "P01", Error),
       test_p02_num => ("TT08/prop_name_num.js", PropertyNameChecker, "P02", Error),
       test_p02_hun => ("TT08/prop_name_hun.js", PropertyNameChecker, "P02", Error),
       test_valid => ("TT08/valid.js", PropertyNameChecker, "", Ok),
    }
}
