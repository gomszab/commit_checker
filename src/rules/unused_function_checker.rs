use crate::api::{Handler, HandlerResult};
use rust_i18n::t;

pub struct UnusedFunctionChecker;

impl Handler for UnusedFunctionChecker {
    fn handle<'a>(&self, context: &'a crate::api::FileContext<'a>) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();
        let scope = semantic.scoping();
        let unused_functions = scope
            .symbol_ids()
            .filter(|id| scope.symbol_is_unused(*id) && scope.symbol_flags(*id).is_function());

        for func_id in unused_functions {
            let span = scope.symbol_span(func_id);
            errors.push(t!(
                "F03", line =
                context.get_line(span.start), function =
                context.lines[context.get_line(span.start) - 1], highlight =
                format!(
                    "{}{}",
                    " ".repeat(context.get_column(span.start) - 1),
                    "^".repeat((span.end - span.start) as usize)
                )
            ).to_string());
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self) -> String {
        format!("Minden függvény használva van")
    }
    fn title(&self) -> String {
        format!("Felhasználatlan függvények ellenőrzése...")
    }
}
