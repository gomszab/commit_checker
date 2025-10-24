use crate::api::Handler;

pub struct JsDocChecker;

impl Handler for JsDocChecker {
    fn handle(&self, context: &mut crate::api::Context) -> Result<(), String> {
        let mut in_jsdoc = false;
        let mut jsdoc = String::new();
        for line in &context.file_contents {
            let trimmed = line.trim();
            if !in_jsdoc && trimmed.starts_with("/**") {
                in_jsdoc = true;
                jsdoc = String::from("/**");
                continue;
            }
            if !(trimmed.contains("/**") || trimmed.contains("*/")) {
                jsdoc += trimmed.strip_prefix('*').unwrap_or(trimmed).trim();
            }

            if in_jsdoc && line.contains("*/") {
                in_jsdoc = false;
                jsdoc.push_str("*/");
                jsdoc = jsdoc.replace(" ", "");
                if jsdoc.contains("{Object}")
                    || jsdoc.contains("{any}")
                    || jsdoc.contains("{*}")
                    || jsdoc.contains("{Array}")
                {
                    return context.end_of_handle(Some(&format!(
                        "Nem megengedett tipus a jsdocban (any, Object, *)"
                    )));
                }
                if jsdoc.contains("@type") {
                    //todo get from config file
                    let re = regex::Regex::new(r"^/\*\*@type\{[^}]+\}[a-zA-Z0-9]+\*/$").unwrap();
                    if !re.is_match(&jsdoc) {
                        return context.end_of_handle(Some(&format!("Nem minden @type jsdoc felel meg a `@type {{tipus}} leiras` formatumnak")));
                    }
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
