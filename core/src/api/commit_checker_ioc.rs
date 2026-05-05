use std::pin::Pin;

use commit_checker_message_handler::{MessageHandler, message_handler::MessageOutput};

use crate::rules::api::RuleHandler;

pub struct CommitCheckerIoC<'a> {
    pub rule_handler: Pin<Box<RuleHandler>>,
    pub message_handler: MessageHandler<'a>,
}

impl<'a> CommitCheckerIoC<'a> {
    pub fn new(out: &'a mut dyn MessageOutput) -> Self {
        Self {
            rule_handler: RuleHandler::new(),
            message_handler: MessageHandler::build(out),
        }
    }
}
