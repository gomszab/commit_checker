use crate::api::{Handler, HandlerResult};

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
                    errors.push(format!(
                        "sor: {}: A @typedef-nek nincs se típus, se név megadva\n{}\n{}",
                        context.get_line(tag.span.start),
                        context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                        "^".repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                    ));
                    continue;
                } else if name_part.parsed().len() == 0 {
                    errors.push(format!(
                        "sor: {}: A @typedef-nek nincs név megadva\n{}\n{}",
                        context.get_line(tag.span.start),
                        context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                        "^".repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                    ));

                    continue;
                } else if let None = type_part {
                    errors.push(format!(
                        "sor: {}: A @typedef-nek nincs típus megadva\n{}\n{}",
                        context.get_line(tag.span.start),
                        context.lines[context.get_line(jsdoc.span.start) - 1
                            ..=context.get_line(jsdoc.span.end) - 1]
                            .to_vec()
                            .join("\n"),
                        "^".repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
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
        format!("Typedef-ek rendben")
    }
    fn title(&self) -> String {
        format!("Typedef-ek analizálása...")
    }
}
