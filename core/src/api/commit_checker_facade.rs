use commit_checker_message_handler::message_handler::MessageOutput;
use oxc::allocator::Allocator;

use crate::{api::commit_checker_ioc::CommitCheckerIoC, rules::api::FileContext};

pub struct CommitCheckerFacade<'a> {
    container: CommitCheckerIoC<'a>,
    enabled_handlers: Option<Vec<String>>,
}

impl<'a> CommitCheckerFacade<'a> {
    pub fn build(out: &'a mut dyn MessageOutput) -> Self {
        CommitCheckerFacade {
            container: CommitCheckerIoC::new(out),
            enabled_handlers: None,
        }
    }

    pub fn build_with_enabled(
        out: &'a mut dyn MessageOutput,
        enabled_handlers: Vec<String>,
    ) -> Self {
        CommitCheckerFacade {
            container: CommitCheckerIoC::new(out),
            enabled_handlers: Some(enabled_handlers),
        }
    }

    pub fn analyze(&mut self, file_name: &str, file_contents: &str) {
        let allocator = Allocator::new();
        let file_context = FileContext::new(
            file_name.to_string(),
            file_contents,
            &allocator,
            &mut self.container.message_handler,
        );
        if let Ok(file_context) = &file_context {
            let handlers = &self.container.rule_handler.handlers.clone();
            for i in 0..handlers.len() {
                let handler = handlers[i].clone();
                if let Some(enabled) = &self.enabled_handlers
                    && !enabled.contains(&handler.code().to_string())
                {
                    continue;
                }
                handler.title(&mut self.container);
                if let crate::rules::api::HandlerResult::Ok =
                    handler.handle(file_context, &mut self.container)
                {
                    handler.success_message(&mut self.container);
                }
            }
        };
    }
}
