use crate::api::{Handler, HandlerResult};
use rust_i18n::t;

pub struct TypeJsDocChecker;

impl Handler for TypeJsDocChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();

        for jsdoc in semantic.jsdoc().iter_all() {
            if let Some(tag) = jsdoc.tags().iter().find(|tag| tag.kind.parsed() == "type") {
                let type_comment = tag.type_comment();
                let ty = type_comment.0;
                let comment = type_comment.1.parsed();

                let start = tag.span.start;
                let line_num = context.get_line(start);
                let comment_span = type_comment.1.span;
                if ty.is_none() && comment.is_empty() {
                    errors.push(t!(
                        "TD01", line =
                        line_num,jsdoc =
                        context.lines[line_num - 1], highlight =
                        format_args!(
                            "{}^",
                            " ".repeat(context.get_column(comment_span.start) - 1),
                        ),
                    ).to_string());
                    continue;
                } else if type_comment.0.is_none() {
                    errors.push(t!(
                        "TD02", line =
                        line_num, jsdoc =
                        context.lines[line_num - 1], highlight =
                        format_args!(
                            "{}^",
                            " ".repeat(context.get_column(comment_span.start) - 1),
                        )
                    ).to_string());
                    continue;
                } else if comment.is_empty() {
                    // We can unwrap because if we get here, ty is not None
                    let ty_span = ty.unwrap().span;
                    errors.push(t!(
                        "TD03", line =
                        line_num, jsdoc =
                        context.lines[line_num - 1], highlight =
                        format_args!("{}^", " ".repeat(context.get_column(ty_span.end) - 1),)
                    ).to_string());
                    continue;
                }
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }
    fn success_message(&self) -> String {
        t!("SCM09").to_string()
    }
    fn title(&self) -> String {
        t!("TT09").to_string()
    }
}
