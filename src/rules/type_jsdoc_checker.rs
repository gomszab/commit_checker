use crate::api::{Handler, HandlerResult};

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
                    errors.push(format!(
                        "sor: {}: A @type JSDoc-nak nincs se típus, se leírás megadva\n{}\n{}",
                        line_num,
                        context.lines[line_num - 1],
                        format_args!(
                            "{}^",
                            " ".repeat(context.get_column(comment_span.start) - 1),
                        ),
                    ));
                    continue;
                } else if type_comment.0.is_none() {
                    errors.push(format!(
                        "sor: {}: A @type JSDoc-nak nincs típus megadva\n{}\n{}",
                        line_num,
                        context.lines[line_num - 1],
                        format_args!(
                            "{}^",
                            " ".repeat(context.get_column(comment_span.start) - 1),
                        )
                    ));
                    continue;
                } else if comment.is_empty() {
                    // We can unwrap because if we get here, ty is not None
                    let ty_span = ty.unwrap().span;
                    errors.push(format!(
                        "sor: {}: A @type JSDoc-nak nincs leírás megadva\n{}\n{}",
                        line_num,
                        context.lines[line_num - 1],
                        format_args!("{}^", " ".repeat(context.get_column(ty_span.end) - 1),)
                    ));
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
        format!("Type-ok rendben")
    }
    fn title(&self) -> String {
        format!("Type-ok analizálása...")
    }
}
