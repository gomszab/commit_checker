use commit_checker_core::api::commit_checker_facade;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmCommitChecker {
    facade: commit_checker_facade::CommitCheckerFacade,
}

#[wasm_bindgen]
impl WasmCommitChecker {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmCommitChecker {
        WasmCommitChecker {
            facade: commit_checker_facade::CommitCheckerFacade::build(),
        }
    }

    pub fn analyze(&mut self, file_name: String, file_contents: String) -> Vec<String>{
        self.facade.analyze(&file_name, &file_contents)
       
    }
}
