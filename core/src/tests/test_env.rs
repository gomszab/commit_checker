use std::{fs, path::PathBuf, sync::{Arc, Mutex}};

use oxc::allocator::Allocator;

pub struct TestEnv {
    pub source_path: String,
    pub file_contents: String,
    pub allocator: Allocator,
}

pub struct TestOut {
    pub messages: Arc<Mutex<Vec<String>>>,
}

impl TestOut {
    pub fn new(messages: Arc<Mutex<Vec<String>>>) -> Self {
        Self {
            messages,
        }
    }
}

impl commit_checker_message_handler::MessageOutput for TestOut {
    
    fn push(&self, _ctx: Option<&commit_checker_message_handler::message_handler::MessageContext>, message: commit_checker_message_handler::message_handler::LocalizedMessage) {
        self.messages.lock().unwrap().push(message.title)
    }
}

impl TestEnv {
    pub fn new(source_path: &str) -> Self {
        Self {
            source_path: source_path.to_string(),
            file_contents: read_file_from_test_folder(source_path),
            allocator: Allocator::new(),
        }
    }

    pub fn build<'a>(
        &'a mut self,
    ) -> (
        crate::rules::api::FileContext<'a>,
        crate::api::commit_checker_ioc::CommitCheckerIoC,
    ) {
        let mut ioc = crate::api::commit_checker_ioc::CommitCheckerIoC::new();
        ioc.rule_handler = crate::rules::api::RuleHandler::new_empty();

        let context = crate::rules::api::FileContext::new(
            self.source_path.clone(),
            &self.file_contents,
            &self.allocator,
        )
        .unwrap_or_else(|err| panic!("Failed to create file context: {}", err));

        (context, ioc)
    }
}

fn read_file_from_test_folder(path_inside_test_folder: &str) -> String {
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

#[macro_export]
macro_rules! declare_tests {
    (
        $(
            $test_name:ident => ($path:expr, $checker:ident, $code:expr, $resulttype:ident)
        ),* $(,)?
    ) => {
        $(
            #[test]
            fn $test_name() {
                use super::*;
                use HandlerResult::*;

                let messages = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
                let _ = commit_checker_message_handler::init_message_handler!({
                    let messages = std::sync::Arc::clone(&messages);    
                    ||
                    crate::tests::TestOut::new(messages)
                });
                let mut env = crate::tests::TestEnv::new($path);
                let (context, mut _ioc) = env.build();

                let checker = $checker;
                let result = checker.handle(&context);
                match $resulttype {
                    HandlerResult::Error => {

                        let test = &messages.lock().unwrap();
                        assert_eq!(test.as_slice(), vec![$code]);
                        crate::assertHandlerResultError!(result);
                    },
                    HandlerResult::Ok => {
                        let vec: Vec<String> = Vec::new();
                        let test = messages.lock().unwrap();
                        assert_eq!(test.as_slice(), vec);
                        crate::assertHandlerResultOk!(result);
                    }
                }
                commit_checker_message_handler::message_impl::clear_message_api()
            }
        )*
    };
}
