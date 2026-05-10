use std::sync::{Arc, RwLock};

use crate::{
    handling_message,
    message_handler::MessageContext,
    IncommingMessage,
    MessageOutput,
};

static MESSAGE_HANDLER: RwLock<Option<Arc<dyn MessageOutput>>> = RwLock::new(None);

pub struct MessageApi;

impl MessageApi {
    pub fn handle(message: IncommingMessage, ctx: Option<&MessageContext>) {
        let guard = MESSAGE_HANDLER
            .read()
            .expect("MESSAGE_HANDLER lock poisoned");

        let handler = guard
            .as_ref()
            .expect("Please init the MESSAGE_HANDLER by `init_message_api` function");

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

    let mut guard = MESSAGE_HANDLER
        .write()
        .expect("MESSAGE_HANDLER lock poisoned");

    *guard = Some(Arc::new(factory_method()));
    Ok(())
}

pub fn clear_message_api() {
    let mut guard = MESSAGE_HANDLER
        .write()
        .expect("MESSAGE_HANDLER lock poisoned");

    *guard = None;
}