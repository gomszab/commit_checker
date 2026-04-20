use commit_checker_message_handler::MessageHandlerApi;
use oxc::allocator::Allocator;

use crate::{api::commit_checker_ioc::CommitCheckerIoC, rules::api::FileContext};

pub struct CommitCheckerFacade {
    container: CommitCheckerIoC,
}

impl CommitCheckerFacade {
    pub fn build() -> Self {
        CommitCheckerFacade {
            container: CommitCheckerIoC::new(),
        }
    }

    pub fn analyze(&mut self, file_name: &str, file_content: &str) -> Vec<String> {
        let allocator = Allocator::new();
        let file_context = FileContext::new(
            file_name.to_string(),
            file_content,
            &allocator,
            &mut self.container.message_handler,
        );
        if let Ok(file_context) = &file_context {
            let handlers = &self.container.rule_handler.handlers.clone();
            for i in 0..handlers.len() {
                let handler = handlers[i].clone();
                handler.title(&mut self.container);
                if let crate::rules::api::HandlerResult::Ok =
                    handler.handle(file_context, &mut self.container)
                {
                    handler.success_message(&mut self.container);
                }
            }
        }
        self.container.message_handler.get_all_messages()
    }
}
