use commit_checker_message_handler::MessageHandler;
use oxc::allocator::Allocator;

use crate::api::commit_checker_ioc::CommitCheckerIoC;

pub struct CommitCheckerFacade {
    container: CommitCheckerIoC,
}

impl CommitCheckerFacade {
    pub fn build() -> Self {
        CommitCheckerFacade { container: CommitCheckerIoC::new() }
    }

    pub fn analyze(&self, file_name: &str, file_content: &str){
        // iterate through handlers, 
    }
}