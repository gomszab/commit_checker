use commit_checker_message_handler::{info_message, rule_success_message, validation_error};

use crate::rules::api::{Handler, HandlerResult};

pub struct UnusedFunctionChecker;

///
/// FIXME: Not used
///
impl Handler for UnusedFunctionChecker {
    fn handle<'a>(
        &self,
        context: &'a crate::rules::api::FileContext<'a>,
        ioc: &mut crate::api::CommitCheckerIoC,
    ) -> HandlerResult {
        let semantic = context.semantic.get().unwrap();
        let scope = semantic.scoping();
        let unused_functions = scope
            .symbol_ids()
            .filter(|id| scope.symbol_is_unused(*id) && scope.symbol_flags(*id).is_function());

        for func_id in unused_functions {
            let span = scope.symbol_span(func_id);
            validation_error!(
                &mut ioc.message_handler,
                "F03",
                &context.file_name,
                context.get_line(span.start) as usize,
                context.get_column(span.start) - 1,
                span.end as usize,
                function = context.lines[context.get_line(span.start) - 1],
            );
        }

        HandlerResult::Ok
    }

    fn success_message(&self, ioc: &mut crate::api::CommitCheckerIoC) {
        rule_success_message!(&mut ioc.message_handler, "SCM11");
    }

    fn code(&self) -> &'static str {
        "TT11"
    }
}
