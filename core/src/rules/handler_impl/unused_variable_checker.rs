use commit_checker_message_handler::{rule_success_message, validation_error};

use crate::rules::api::{Handler, HandlerResult};

pub struct UnusedVariableChecker;

impl Handler for UnusedVariableChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>
    ) -> HandlerResult {
        let mut result = HandlerResult::Ok;
        let semantic = context.semantic.get().unwrap();
        let scope = semantic.scoping();
        let unused_variables = scope
            .symbol_ids()
            .filter(|id| scope.symbol_is_unused(*id) && scope.symbol_flags(*id).is_variable());

        for var_id in unused_variables {
            let span = scope.symbol_span(var_id);
            validation_error!(
                "V01",
                context.get_line(span.start) as usize,
                context.get_column(span.start) - 1,
                span.end as usize,
                variable = context.lines[context.get_line(span.start) - 1],
            );
            result = HandlerResult::Error;
        }

        result
    }

    fn success_message(&self) {
        rule_success_message!("SCM12");
    }

    fn code(&self) -> &'static str {
        "TT12"
    }
}

#[cfg(test)]
mod tests {

    crate::declare_tests! {
       test_v01 => ("TT12/notused_var.js", UnusedVariableChecker, "V01", Error),
       test_valid => ("TT12/usedvar.js", UnusedVariableChecker, "", Ok),
    }
}
