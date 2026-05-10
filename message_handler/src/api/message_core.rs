use std::{
    cell::RefCell,
};


use crate::{ message_handler::{
    IncommingMessage, LocalizedMessage, MessageContext
}, message_impl::MessageApi};

thread_local! { // no overhead in case of single thread
    static MESSAGE_CONTEXT: RefCell<Option<MessageContext>> =  RefCell::new(None);
}

pub trait MessageOutput: Send + Sync + 'static {
    fn push(&self, ctx: Option<&MessageContext>, message: LocalizedMessage);
}


pub fn change_message_context(ctx: MessageContext) {
    MESSAGE_CONTEXT.replace(Some(ctx));
}

pub fn handle_with_context(message: IncommingMessage){
    MESSAGE_CONTEXT.with(|slot| {
        MessageApi::handle(message, slot.borrow().as_ref());
    });
}
