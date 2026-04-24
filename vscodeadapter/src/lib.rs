use commit_checker_core::{api::commit_checker_facade::CommitCheckerFacade, message_handler::MessageOutput};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmCommitChecker {
    
}

#[wasm_bindgen]
impl WasmCommitChecker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmCommitChecker {
        WasmCommitChecker {
            
        }
    }

    pub fn analyze(&mut self, file_name: String, file_contents: String) -> String {
      let mut out = VsCodeAdapter::new();
      let mut facade = CommitCheckerFacade::build(&mut out);
      facade.analyze(&file_name, &file_contents);
      serde_json::to_string(&out.messages).unwrap()
    }
}

#[derive(Serialize)]
pub struct PluginMessage{
    pub details: String,
    pub row: usize,
    pub column_start: usize,
    pub column_end: usize,
}

struct VsCodeAdapter{
    messages: Vec<PluginMessage>
}

impl VsCodeAdapter{
    fn new()-> Self{
        Self {
            messages: Vec::new()
        }
    }
}

impl MessageOutput for VsCodeAdapter{
    fn push(&mut self, message: commit_checker_core::message_handler::LocalizedMessage) {
        let typ = message.typ;
       let message=  match typ {
            commit_checker_core::message_handler::MessageType::ValidationError => {
                let details = message.details.to_string();
                let meta = &message.meta.unwrap();
                let row = meta.row.unwrap();
                let (_, right) = details.split_once('|').unwrap();
                let (middle, _) = right.split_once('\n').unwrap();

                let details = middle.trim();
                let column_start = meta.column_start.unwrap();
                let column_end = meta.column_end.unwrap();
                Some(PluginMessage{
                    column_end,
                    column_start,
                    details: details.to_string(),
                    row,
                })
            },
            commit_checker_core::message_handler::MessageType::Error | commit_checker_core::message_handler::MessageType::Success | commit_checker_core::message_handler::MessageType::Info => {
                None
            }
        };
        if let Some(message) = message {
            self.messages.push(message);
        }
    }
}