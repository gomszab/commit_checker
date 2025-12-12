use crate::api::Handler;

pub struct JsDocChecker;

impl Handler for JsDocChecker {
    fn handle(&self, context: &mut crate::api::Context) -> Result<(), String> {
        let mut in_jsdoc = false;
        for (line_number, line) in context.file_contents.iter().enumerate() {
            let trimmed = line.trim();
            if !in_jsdoc && trimmed.starts_with("/**") {
                in_jsdoc = true;
                continue;
            }

            if trimmed.contains("*/") {
                in_jsdoc = false;
            }

            if trimmed.contains("{Object}")
                || trimmed.contains("{any}")
                || trimmed.contains("{*}")
                || trimmed.contains("{Array}")
            {
                context.errors.push(format!(
                    "sor: {}:  Nem megengedett tipus a jsdocban (any, Object, *)",
                    line_number + 1
                ));
            }

            if trimmed.contains("@type") {
                //todo get from config file
                let re = regex::Regex::new(r#"^*\*\s*@type\s*\{[^}]+\}.+$"#).unwrap();
                if !re.is_match(trimmed) {
                    context.errors.push(format!(
                        "sor: {}: A @type jsdoc nem felel meg a `@type {{tipus}} leiras` formatumnak",
                        line_number + 1
                    ));
                }
            }
        }
        context.end_of_handle(None)
    }
    fn success_message(&self) -> String {
        format!("Jsdoc latszolag rendben")
    }
    fn title(&self) -> String {
        format!("Jsdoc analizalasa folyamatban")
    }
}
