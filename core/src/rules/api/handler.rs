use commit_checker_message_handler::info_message;

use crate::{api::CommitCheckerIoC, rules::api::file_context::FileContext};

pub trait Handler {
    fn title(&self, ioc: &mut CommitCheckerIoC) {
        info_message!(&mut ioc.message_handler, self.code());
    }
    fn handle<'a>(&self, context: &'a FileContext<'a>, ioc: &mut CommitCheckerIoC)
    -> HandlerResult;
    fn success_message(&self, ioc: &mut CommitCheckerIoC);

    fn code(&self) -> &'static str;
}

pub enum HandlerResult {
    Ok,
    Error,
}
