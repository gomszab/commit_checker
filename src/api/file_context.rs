use std::rc::Rc;

use crate::api::{Handler, HandlerResult};
use line_numbers::LinePositions;
use oxc::{allocator::Allocator, ast::ast::Program, parser::Parser, span::SourceType};
use oxc_semantic::{Semantic, SemanticBuilder};
use rust_i18n::t;

pub struct FileContext<'a> {
    pub file_name: String,
    pub lines: Vec<&'a str>,
    pub line_positions: LinePositions,
    pub semantic: Semantic<'a>,
    pub program: Box<Program<'a>>,
    handlers: Vec<Rc<dyn Handler>>,
}

impl<'a> FileContext<'a> {
    pub fn new(
        file_name: String,
        file_contents: &'a str,
        allocator: &'a Allocator,
    ) -> Result<Self, String> {
        let parsed = Parser::new(allocator, file_contents, SourceType::mjs()).parse();

        if !parsed.errors.is_empty() {
            return Err(t!("SW04", file_name = file_name).to_string());
        }

        let program = Box::new(parsed.program);
        // SAFETY: The pointer is fine because we just got it.
        let analyzed = SemanticBuilder::new()
            .build(unsafe { (&*program as *const Program).as_ref_unchecked() });

        if !analyzed.errors.is_empty() {
            return Err(t!("SW04", file_name = file_name).to_string());
        }

        Ok(FileContext {
            file_name: file_name.clone(),
            lines: file_contents.lines().collect(),
            line_positions: LinePositions::from(file_contents),
            semantic: analyzed.semantic,
            program,
            handlers: Vec::new(),
        })
    }

    pub fn register_handler(&mut self, handler: Rc<dyn Handler>) {
        self.handlers.push(handler);
    }

    pub fn run(&'a self) -> Result<FileFeedback, String> {
        let mut file_feedback = FileFeedback::new(self.file_name.clone());

        for i in 0..self.handlers.len() {
            let handler = self.handlers[i].clone();
            let mut task_feedback = TestFeedback::new(handler.title());

            let result = handler.handle(self);
            match result {
                HandlerResult::Ok => task_feedback.messages.push(handler.success_message()),
                HandlerResult::Error(mut errors) => {
                    task_feedback.messages.append(&mut errors);
                    task_feedback.errored = true;
                }
            };

            file_feedback.tasks.push(task_feedback);
        }

        Ok(file_feedback)
    }

    pub fn get_line(&self, offset: u32) -> usize {
        self.line_positions.from_offset(offset as usize).0.0 as usize + 1
    }

    pub fn get_column(&self, offset: u32) -> usize {
        self.line_positions.from_offset(offset as usize).1 as usize + 1
    }
}

// For storing feedbacks from FileContext
pub struct FileFeedback {
    pub file_name: String,
    pub tasks: Vec<TestFeedback>,
}

impl FileFeedback {
    pub fn new(file_name: String) -> FileFeedback {
        FileFeedback {
            file_name,
            tasks: Vec::new(),
        }
    }
}

pub struct TestFeedback {
    pub task_name: String,
    pub messages: Vec<String>,
    pub errored: bool,
}

impl TestFeedback {
    pub fn new(task_name: String) -> TestFeedback {
        TestFeedback {
            task_name,
            messages: Vec::new(),
            errored: false,
        }
    }
}
