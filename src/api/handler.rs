use crate::api::FileContext;

pub trait Handler {
    fn title(&self) -> String;
    fn handle<'a>(&self, context: &'a FileContext<'a>) -> HandlerResult;
    fn success_message(&self) -> String;
}

pub enum HandlerResult {
    Ok,
    Error(Vec<String>),
}
