use std::sync::{Arc, OnceLock};

use crate::{IncommingMessage, MessageOutput, handling_message, message_handler::MessageContext};

static MESSAGE_HANDLER: OnceLock<Arc<dyn MessageOutput>> = OnceLock::new();

pub struct MessageApi;

impl MessageApi {
    pub fn handle(message: IncommingMessage, ctx: Option<&MessageContext>) {
        let handler = MESSAGE_HANDLER.get().expect("Please init the MESSAGE_HANDLER by `init_message_handler` function");
        handling_message(message, ctx, handler.as_ref());
    }
}

pub fn init_message_api<Factory, InstanceType>(
    factory_method: Factory,
    language: Option<&str>,
) -> Result<(), Arc<dyn MessageOutput>>
where
    Factory: FnOnce() -> InstanceType,
    InstanceType: MessageOutput,
{
    let language = language.unwrap_or("hu");
    rust_i18n::set_locale(language);
    // set Handler
    MESSAGE_HANDLER.set(Arc::new(factory_method()))
}