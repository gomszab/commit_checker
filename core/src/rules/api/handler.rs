use crate::{api::CommitCheckerIoC, rules::api::file_context::FileContext};

pub trait Handler {
    fn title(&self) -> String;
    fn handle<'a>(&self, context: &'a FileContext<'a>, ioc: &CommitCheckerIoC) -> HandlerResult;
    fn success_message(&self) -> String;
}

pub enum HandlerResult {
    Ok,
    Error(Vec<String>),
}