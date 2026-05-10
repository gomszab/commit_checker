use commit_checker_message_handler::info_message;

use crate::{rules::api::file_context::FileContext};

pub trait Handler {
    fn title(&self) {
        info_message!(self.code());
    }
    fn handle<'a>(&self, context: &'a FileContext<'a>)
    -> HandlerResult;
    fn success_message(&self);

    fn code(&self) -> &'static str;
}

pub enum HandlerResult {
    Ok,
    Error,
}
