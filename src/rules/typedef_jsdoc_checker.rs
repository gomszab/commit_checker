use crate::api::{Handler, HandlerResult};
use rust_i18n::t;

pub struct TypedefJsDocChecker;

impl Handler for TypedefJsDocChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();

        for jsdoc in semantic.jsdoc().iter_all() {
            #[allow(for_loops_over_fallibles)]
            for tag in jsdoc
                .tags()
                .iter()
                .find(|tag| tag.kind.parsed() == "typedef")
            {
                let (type_part, name_part) = tag.type_comment();

                if let None = type_part
                    && name_part.parsed().len() == 0
                {
                    errors.push(
                        t!(
                            "TD04",
                            line = context.get_line(tag.span.start),
                            jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                                ..=context.get_line(jsdoc.span.end) - 1]
                                .to_vec()
                                .join("\n"),
                            highlight = "^"
                                .repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                        )
                        .to_string(),
                    );
                    continue;
                } else if name_part.parsed().len() == 0 {
                    errors.push(
                        t!(
                            "TD05",
                            line = context.get_line(tag.span.start),
                            jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                                ..=context.get_line(jsdoc.span.end) - 1]
                                .to_vec()
                                .join("\n"),
                            highlight = "^"
                                .repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                        )
                        .to_string(),
                    );

                    continue;
                } else if let None = type_part {
                    errors.push(
                        t!(
                            "TD06",
                            line = context.get_line(tag.span.start),
                            jsdoc = context.lines[context.get_line(jsdoc.span.start) - 1
                                ..=context.get_line(jsdoc.span.end) - 1]
                                .to_vec()
                                .join("\n"),
                            highlight = "^"
                                .repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                        )
                        .to_string(),
                    );
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
        t!("SCM10").to_string()
    }
    fn title(&self) -> String {
        t!("TT10").to_string()
    }
}
