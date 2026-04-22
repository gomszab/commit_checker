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
      let list: Vec<&PluginMessage> = out.messages.iter().filter(|item| !item.ignored).collect();
      serde_json::to_string(&list).unwrap()
    }
}

#[derive(Serialize)]
pub struct PluginMessage{
    pub details: String,
    pub row: usize,
    pub column_start: usize,
    pub column_end: usize,
    ignored: bool
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
                let (_, details) = details.split_at(14);
                let column_start = meta.column_start.unwrap();
                let column_end = meta.column_end.unwrap();
                PluginMessage{
                    column_end,
                    column_start,
                    details: details.to_string(),
                    row,
                    ignored: false
                }
            },
            commit_checker_core::message_handler::MessageType::Error | commit_checker_core::message_handler::MessageType::Success | commit_checker_core::message_handler::MessageType::Info => {
                PluginMessage{
                    column_start: 0_usize,
                    column_end: 1_usize,
                    details: message.details,
                    row: 0_usize,
                    ignored: true
                }
            }
        };
        self.messages.push(message);
    }
}