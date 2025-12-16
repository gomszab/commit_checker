use crate::api::{Handler, HandlerResult};

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
            errors.push(format!(
                "sor: {}: Felhasználatlan függvény\n{}\n{}",
                context.get_line(span.start),
                context.lines[context.get_line(span.start) - 1],
                format!(
                    "{}{}",
                    " ".repeat(context.get_column(span.start) - 1),
                    "^".repeat((span.end - span.start) as usize)
                )
            ));
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
