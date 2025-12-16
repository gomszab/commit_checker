use crate::api::{Handler, HandlerResult};

pub struct JsDocTypeChecker;

// TODO: maybe get it from a config file?
const FORBIDDEN_TYPES: [&str; 3] = ["Object", "Array", "*"];

impl Handler for JsDocTypeChecker {
    fn handle<'a>(&self, context: &'a crate::api::FileContext<'a>) -> HandlerResult {
        let mut errors = Vec::new();
        let semantic = context.semantic.get().unwrap();

        for jsdoc in semantic.jsdoc().iter_all() {
            for tag in jsdoc.tags() {
                let Some(type_part) = tag.r#type() else {
                    continue;
                };

                let ty = type_part.parsed();
                let Some(found_forbidden) = FORBIDDEN_TYPES.iter().find(|s| ty.contains(*s)) else {
                    continue;
                };

                errors.push(format!(
                    "sor: {}: A(z) {found_forbidden} használata típusként nem megengedett\n{}\n{}",
                    context.get_line(tag.span.start),
                    context.lines[context.get_line(jsdoc.span.start) - 1
                        ..=context.get_line(jsdoc.span.end) - 1]
                        .to_vec()
                        .join("\n"),
                    "^".repeat(context.lines[context.get_line(jsdoc.span.end - 2)].len())
                ));
            }
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::Error(errors)
        }
    }

    fn success_message(&self) -> String {
        format!("A JSDoc típusok rendben")
    }
    fn title(&self) -> String {
        format!("JSDoc típusok ellenőrzése...")
    }
}
