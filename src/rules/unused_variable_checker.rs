use crate::api::{Handler, HandlerResult};
use rust_i18n::t;

pub struct UnusedVariableChecker;

impl Handler for UnusedVariableChecker {
    fn handle<'a>(&self, context: &'a crate::api::FileContext<'a>) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        let scope = semantic.scoping();
        let unused_variables = scope
            .symbol_ids()
            .filter(|id| scope.symbol_is_unused(*id) && scope.symbol_flags(*id).is_variable());

        for var_id in unused_variables {
            let span = scope.symbol_span(var_id);
            errors.push(
                t!(
                    "V01",
                    line = context.get_line(span.start),
                    variable = context.lines[context.get_line(span.start) - 1],
                    highlight = format!(
                        "{}{}",
                        " ".repeat(context.get_column(span.start) - 1),
                        "^".repeat((span.end - span.start) as usize)
                    )
                )
                .to_string(),
            );
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self) -> String {
        t!("SCM12").to_string()
    }
    fn title(&self) -> String {
        t!("TT12").to_string()
    }
}
