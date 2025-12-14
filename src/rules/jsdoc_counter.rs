use crate::api::{Handler, HandlerResult};

pub struct JsDocCounter;

impl Handler for JsDocCounter {
    fn handle(&self, context: &mut crate::api::Context) -> HandlerResult {
        let mut errors = Vec::new();
        let mut definition_counter = 0;
        let mut jsdoc_counter = 0;

        for line in &context.file_contents {
            let line = line.trim();
            if is_definition_line(line) {
                definition_counter += 1;
            }
            if line.contains("/**") {
                jsdoc_counter += 1;
            }
        }

        if definition_counter != jsdoc_counter {
            errors.push(format!(
                "Nincs minden valtozo deklaracio es fuggvenydefinicio dokumentalva. definiciok szama: {}, jsdoc: {}",
                definition_counter, jsdoc_counter
            ));
        }

        if errors.is_empty() {
            HandlerResult::Ok
        } else {
            HandlerResult::SoftErrors(errors)
        }
    }

    fn success_message(&self) -> String {
        format!("JsDocok szama rendben")
    }
    fn title(&self) -> String {
        format!("JSDoc szamolas folyamatban")
    }
}

pub fn is_definition_line(line: &str) -> bool {
    let definition_keywords = vec!["const", "let", "var", "function"];
    for keyword in definition_keywords {
        if line.starts_with(keyword) {
            return true;
        }
    }
    return false;
}
