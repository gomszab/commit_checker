use crate::{api::CommitCheckerIoC, rules::api::file_context::FileContext};

pub trait Handler {
    fn title(&self, ioc: &mut CommitCheckerIoC);
    fn handle<'a>(&self, context: &'a FileContext<'a>, ioc: &mut CommitCheckerIoC)
    -> HandlerResult;
    fn success_message(&self, ioc: &mut CommitCheckerIoC);
}

pub enum HandlerResult {
    Ok,
    Error,
}
