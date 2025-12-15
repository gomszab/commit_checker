use crate::api::{Handler, HandlerResult};

pub struct TypedefJsDocChecker;

impl Handler for TypedefJsDocChecker {
    fn handle(&self, context: &crate::api::FileContext) -> HandlerResult {
        let mut errors = Vec::new();

        for jsdoc in context.semantic.get().unwrap().jsdoc().iter_all() {
            let start_line = context.get_line(jsdoc.span.start);
            let end_line = context.get_line(jsdoc.span.end);
            for (relative_line_number, line) in
                (&context.lines[start_line..end_line]).iter().enumerate()
            {
                let trimmed = line.trim();

                if trimmed.contains("{Object}")
                    || trimmed.contains("{any}")
                    || trimmed.contains("{*}")
                    || trimmed.contains("{Array}")
                {
                    errors.push(format!(
                        "sor: {}:  Nem megengedett tipus a jsdocban (any, Object, *)",
                        start_line + relative_line_number
                    ));
                }

                if trimmed.contains("@typedef") {
                    //todo get from config file
                    let re = regex::Regex::new(
                        r#"^(?:/\*)?\*\s*@typedef\s*\{\{[^}]+\}\}\s*[^\s\*/]+\s*(?:\s?\*/)?$"#,
                    )
                    .unwrap();
                    if !re.is_match(trimmed) {
                        errors.push(format!(
                        "sor: {}: A @typedef jsdoc nem felel meg a `@typedef {{tipus}} Típusnév` formatumnak",
                        start_line + relative_line_number
                    ));
                    }
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
        format!("Jsdoc latszolag rendben")
    }
    fn title(&self) -> String {
        format!("Jsdoc analizalasa folyamatban")
    }
}
