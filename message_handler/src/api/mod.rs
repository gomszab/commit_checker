pub mod message_handler;
pub mod message_core;
pub mod message_message_mapping;
#[cfg(not(feature = "testmode"))]
#[path="message_prod.rs"]
pub mod message_impl;

#[cfg(feature = "testmode")]
#[path = "message_test.rs"]
pub mod message_impl;

mod message_macros;

pub use message_handler::{IncommingMessage};
pub use message_core::{MessageOutput, change_message_context, handle_with_context};
pub use message_message_mapping::handling_message;