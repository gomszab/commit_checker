use crate::api::Handler;

pub struct JsDocCounter;

impl Handler for JsDocCounter {
    fn handle(&self, context: &mut crate::api::Context) -> Result<(), String> {
        let mut definition_counter = 0;
        let mut jsdoc_counter = 0;

        for line in &context.file_contents {
            if is_definition_line(line) {
                definition_counter += 1;
            }
            if line.contains("/**") {
                jsdoc_counter += 1;
            }
        }

        if definition_counter == jsdoc_counter {
            context.end_of_handle(None)
        } else {
            context.end_of_handle(Some(&format!("Nincs minden valtozo deklaracio es fuggvenydefinicio dokumentalva. definiciok szama: {}, jsdoc: {}", definition_counter, jsdoc_counter)))
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
