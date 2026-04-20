use std::pin::Pin;

use commit_checker_message_handler::MessageHandler;

use crate::rules::api::RuleHandler;

pub struct CommitCheckerIoC {
    pub rule_handler: Pin<Box<RuleHandler>>,
    pub message_handler: MessageHandler,
}

impl CommitCheckerIoC {
    pub fn new() -> Self {
        Self {
            rule_handler: RuleHandler::new(),
            message_handler: MessageHandler::build(),
        }
    }
}
