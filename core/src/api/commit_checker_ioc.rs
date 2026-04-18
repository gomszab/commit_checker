use std::rc::Rc;

use commit_checker_message_handler::MessageHandler;
use oxc::allocator::Allocator;

use crate::rules::api::handler::Handler;

pub struct CommitCheckerIoC{
    handlers: Vec<Rc<dyn Handler>>,
    message_handler: MessageHandler,
}

impl CommitCheckerIoC {
    pub fn new() -> Self{
        CommitCheckerIoC { handlers: Vec::new(), message_handler: MessageHandler::build()}
    }
}