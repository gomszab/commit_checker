use std::sync::{Arc, Mutex};

use commit_checker_core::{
    MessageOutput, api::commit_checker_facade::CommitCheckerFacade, init_message_handler
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmCommitChecker {
    messages: Arc<Mutex<Vec<PluginMessage>>>
}

#[wasm_bindgen]
impl WasmCommitChecker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmCommitChecker {
        let messages = Arc::new(Mutex::new(Vec::new()));
        let _ = init_message_handler!({
            let messages = Arc::clone(&messages);
            move || VsCodeAdapter::new(messages)
        });
        WasmCommitChecker { messages }
    }

    pub fn analyze(&mut self, file_name: String, file_contents: String) -> String {
        {
            let mut guard = self.messages.lock().unwrap();
            guard.clear();
        }
        let mut facade = CommitCheckerFacade::build();
        facade.analyze(&file_name, &file_contents);
        let messages = self.messages.lock().unwrap();
        serde_json::to_string(messages.as_slice()).unwrap()
    }
}

#[derive(Serialize)]
pub struct PluginMessage {
    pub details: String,
    pub row: usize,
    pub column_start: usize,
    pub column_end: usize,
}

struct VsCodeAdapter {
    messages:  Arc<Mutex<Vec<PluginMessage>>>,
}

impl VsCodeAdapter {
    fn new(messages: Arc<Mutex<Vec<PluginMessage>>>) -> Self {
        Self {
            messages,
        }
    }
}

impl MessageOutput for VsCodeAdapter {
    fn push(&self, _ctx: Option<&commit_checker_core::message_handler::MessageContext>, message: commit_checker_core::message_handler::LocalizedMessage) {
             let typ = message.typ;
                let message = match typ {
            commit_checker_core::message_handler::MessageType::ValidationError => {
                let details = message.details.to_string();
                let meta = &message.meta.unwrap();
                let row = meta.row.unwrap();
                let (_, right) = details.split_once('|').unwrap();
                let middle = right.trim();
                let middle = if right.contains('\n') {
                    let (middle, _) = right.split_once('\n').unwrap();
                    middle
                } else {
                    middle
                };

                let details = middle.trim();
                let column_start = meta.column_start.unwrap();
                let column_end = meta.column_end.unwrap();
                Some(PluginMessage {
                    column_end,
                    column_start,
                    details: details.to_string(),
                    row,
                })
            }
            commit_checker_core::message_handler::MessageType::Error => Some(PluginMessage {
                column_end: 2_usize,
                column_start: 0_usize,
                details: message.details.to_string(),
                row: 0_usize,
            }),
            commit_checker_core::message_handler::MessageType::Success
            | commit_checker_core::message_handler::MessageType::Info => None,
        };
        if let Some(message) = message {
            self.messages.lock().unwrap().push(message);
        }
    }
}
