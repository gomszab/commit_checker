use crate::api::Context;

pub trait Handler {
    fn title(&self) -> String;
    fn handle(&self, context: &mut Context) -> Result<(), String>;
    fn success_message(&self) -> String;
}
