use std::pin::Pin;

use crate::rules::api::RuleHandler;

pub struct CommitCheckerIoC {
    pub rule_handler: Pin<Box<RuleHandler>>,
}

impl CommitCheckerIoC {
    pub fn new() -> Self {
        Self {
            rule_handler: RuleHandler::new(),
        }
    }
}
