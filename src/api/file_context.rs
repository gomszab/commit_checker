use std::{cell::OnceCell, marker::PhantomPinned, pin::Pin, rc::Rc};

use colored::Colorize;

use line_numbers::LinePositions;
use oxc::{allocator::Allocator, ast::ast::Program, parser::Parser, span::SourceType};
use oxc_semantic::{Semantic, SemanticBuilder};

use spinoff::{Color, Spinner, spinners};

use crate::api::{Handler, HandlerResult};

pub struct FileContext<'a> {
    pub file_name: String,
    pub lines: Vec<&'a str>,
    pub line_positions: LinePositions,
    // Semantic, then program, for the correct drop order.
    pub semantic: OnceCell<Semantic<'a>>,
    pub program: Program<'a>,
    handlers: Vec<Rc<dyn Handler>>,
    _pin: PhantomPinned,
}

impl<'a> FileContext<'a> {
    pub fn new(
        file_name: String,
        file_contents: &'a str,
        allocator: &'a Allocator,
    ) -> Result<Pin<Box<Self>>, String> {
        let parsed = Parser::new(allocator, file_contents, SourceType::cjs()).parse();

        if !parsed.errors.is_empty() {
            return Err(format!("Hiba van a {file_name} fájlban!"));
        }

        let mut file_context = Box::pin(FileContext {
            file_name: file_name.clone(),
            lines: file_contents.lines().collect(),
            line_positions: LinePositions::from(file_contents),
            semantic: OnceCell::new(),
            program: parsed.program,
            handlers: Vec::new(),
            _pin: PhantomPinned,
        });

        let context_ptr = file_context.as_ref().get_ref() as *const FileContext;
        let analyzed = SemanticBuilder::new()
            // SAFETY: The pointer is fine, as we just got it. Semantic will be dropped before
            // program, because of the declaration order.
            .build(unsafe { &(*context_ptr).program });

        if !analyzed.errors.is_empty() {
            return Err(format!("Hiba van a {file_name} fájlban!"));
        }

        // SAFETY: We don't do anything with the pinned Program and we don't cause any moves.
        unsafe {
            // This should always succeed.
            let _ = file_context
                .as_mut()
                .get_unchecked_mut()
                .semantic
                .set(analyzed.semantic);
        }

        Ok(file_context)
    }

    pub fn register_handler(self: &mut Pin<Box<Self>>, handler: Rc<dyn Handler>) {
        // SAFETY: We only access handlers to push, so we don't move it.
        let handlers = &mut unsafe { self.as_mut().get_unchecked_mut() }.handlers;
        handlers.push(handler);
    }

    pub fn run(&'a self) -> Result<bool, String> {
        let mut errored = false;
        for i in 0..self.handlers.len() {
            let handler = self.handlers[i].clone();
            let message = format!("{}: {}", self.file_name, handler.title());
            let mut spinner = Spinner::new(spinners::Circle, message, Color::Blue);

            // SAFETY: Handlers only get an immutable reference to self, so they can't invalidate
            // any pointers.
            let result = handler.handle(self);
            spinner.stop();

            match result {
                HandlerResult::Ok => {
                    println!("{}: {}", self.file_name, handler.success_message().green())
                }
                HandlerResult::Error(errors) => {
                    errored = true;
                    print_errors(errors);
                }
            };
        }

        Ok(errored)
    }

    pub fn get_line(&self, offset: u32) -> usize {
        self.line_positions.from_offset(offset as usize).0.0 as usize + 1
    }

    pub fn get_column(&self, offset: u32) -> usize {
        self.line_positions.from_offset(offset as usize).1 as usize + 1
    }
}

fn print_errors(errors: Vec<String>) {
    for error in errors {
        eprintln!("{}", error.red());
    }
}
