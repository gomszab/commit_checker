use commit_checker_message_handler::{change_message_context, message_handler::MessageContext};
use oxc::allocator::Allocator;

use crate::{api::commit_checker_ioc::CommitCheckerIoC, rules::api::FileContext};

pub struct CommitCheckerFacade {
    container: CommitCheckerIoC,
    enabled_handlers: Option<Vec<String>>,
}

impl CommitCheckerFacade {
    pub fn build() -> Self {
        CommitCheckerFacade {
            container: CommitCheckerIoC::new(),
            enabled_handlers: None,
        }
    }

    pub fn build_with_enabled(
        enabled_handlers: Vec<String>,
    ) -> Self {
        CommitCheckerFacade {
            container: CommitCheckerIoC::new(),
            enabled_handlers: Some(enabled_handlers),
        }
    }

    pub fn analyze(&mut self, file_name: &str, file_contents: &str) {
        change_message_context(MessageContext{
            file_name: file_name.to_string()
        });
        let allocator = Allocator::new();
        let file_context = FileContext::new(
            file_name.to_string(),
            file_contents,
            &allocator,
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
                handler.title();
                if let crate::rules::api::HandlerResult::Ok =
                    handler.handle(file_context)
                {
                    handler.success_message();
                }
            }
        };
    }
}
