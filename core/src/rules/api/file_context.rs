use std::{cell::OnceCell, marker::PhantomPinned};

use line_numbers::LinePositions;
use oxc::ast::ast::Program;
use oxc_semantic::Semantic;

pub struct FileContext<'a> {
    pub file_name: String,
    pub lines: Vec<&'a str>,
    pub line_positions: LinePositions,
    // Semantic, then program, for the correct drop order.
    pub semantic: OnceCell<Semantic<'a>>,
    pub program: Program<'a>,
    _pin: PhantomPinned,
}