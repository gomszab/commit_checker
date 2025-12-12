use crate::api::Context;

pub trait Handler {
    fn title(&self) -> String;
    fn handle(&self, context: &mut Context) -> HandlerResult;
    fn success_message(&self) -> String;
}

pub enum HandlerResult {
    Ok,
    SoftErrors(Vec<String>),
    FatalError(String),
}
