use std::cell::OnceCell;

use commit_checker_message_handler::{MessageHandler,software_error};
use line_numbers::LinePositions;
use oxc::{allocator::Allocator, ast::ast::Program, parser::Parser, span::SourceType};
use oxc_semantic::{Semantic, SemanticBuilder};

pub struct FileContext<'a> {
    pub file_name: String,
    pub lines: Vec<&'a str>,
    pub line_positions: LinePositions,
    // Semantic, then program, for the correct drop order.
    pub semantic: OnceCell<Semantic<'a>>,
    pub program: Program<'a>,
}

impl<'a> FileContext<'a> {
    pub fn new(
        file_name: String,
        file_contents: &'a str,
        allocator: &'a Allocator,
        handler: &mut MessageHandler,
    ) -> Result<Self, String> {
        let parsed = Parser::new(&allocator, file_contents, SourceType::mjs()).parse();

        if !parsed.errors.is_empty() {
            software_error!(handler, "SW04", Some(file_name.to_string()));
            return Err("Error Happened".to_string());
        }

        let file_context = FileContext {
            file_name: file_name.clone(),
            lines: file_contents.lines().collect(),
            line_positions: LinePositions::from(file_contents),
            semantic: OnceCell::new(),
            program: parsed.program,
        };

        let context_ptr = &file_context as *const FileContext;
        let analyzed = SemanticBuilder::new()
            // SAFETY: The pointer is fine, as we just got it. Semantic will be dropped before
            // program, because of the declaration order.
            .build(unsafe { &(*context_ptr).program });

        if !analyzed.errors.is_empty() {
            software_error!(handler, "SW04", Some(file_name.to_string()));
            return Err("error happened".to_string());
        }

        // This should always succeed.
        let _ = file_context.semantic.set(analyzed.semantic);

        Ok(file_context)
    }

    pub fn get_line(&self, offset: u32) -> usize {
        self.line_positions.from_offset(offset as usize).0.0 as usize + 1
    }

    pub fn get_column(&self, offset: u32) -> usize {
        self.line_positions.from_offset(offset as usize).1 as usize + 1
    }
}
