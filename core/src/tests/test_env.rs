use std::{fs, path::PathBuf};

use oxc::allocator::Allocator;

use super::*;

pub struct TestEnv {
    pub source_path: String,
    pub file_contents: String,
    pub allocator: Allocator,
    pub test_out: TestOut,
}

pub struct TestOut {
   pub messages: Vec<String>
}

impl TestOut{
    fn new() -> Self{
        Self { messages: Vec::new() }
    }
}

impl commit_checker_message_handler::message_handler::MessageOutput for TestOut {
    fn push(&mut self, message: commit_checker_message_handler::message_handler::LocalizedMessage) {
        self.messages.push(message.title)
    }
}

impl TestEnv {
    pub fn new(source_path: &str) -> Self {
        Self {
            source_path: source_path.to_string(),
            file_contents: read_file_from_test_folder(source_path),
            allocator: Allocator::new(),
            test_out: TestOut::new(),
        }
    }

    pub fn build<'a>(&'a mut self) -> (
        crate::rules::api::FileContext<'a>,
        crate::api::CommitCheckerIoC<'a>
    ) {
        let mut ioc = crate::api::CommitCheckerIoC::new(&mut self.test_out);
        ioc.rule_handler = crate::rules::api::RuleHandler::new_empty();

        let context = crate::rules::api::FileContext::new(
            self.source_path.clone(),
            &self.file_contents,
            &self.allocator,
            &mut ioc.message_handler,
        )
        .unwrap_or_else(|err| panic!("Failed to create file context: {}", err));

        (context, ioc)
    }
}


fn read_file_from_test_folder(path_inside_test_folder: &str) -> String{
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("tests")
        .join(path_inside_test_folder);

    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Failed to read test file '{}': {}", path.display(), err))
}

#[macro_export]
macro_rules! assertHandlerResultOk {
    ($result:expr $(,)?) => {
        match $result {
            HandlerResult::Ok => {
                // nothing to do, assertion passed
            }
            _ => {
                panic!("expected HandlerResult::Ok, got HandlerResult::Error");
            }
        }
    };
}

#[macro_export]
macro_rules! assertHandlerResultError {
    ($result:expr $(,)?) => {
        match $result {
            HandlerResult::Error => {
                // nothing to do, assertion passed
            }
            _ => {
                panic!("expected HandlerResult::Error, got HandlerResult::Ok");
            }
        }
    };
}