use regex::Regex;

use crate::{api::Handler, rules::jsdoc_counter::is_definition_line};

pub struct VariableNameChecker;

impl Handler for VariableNameChecker {
    fn handle(&self, context: &mut crate::api::Context) -> Result<(), String> {
        for line in &context.file_contents {
            if is_definition_line(line) {
                let re = Regex::new(
                    r"(?:(?:const|let|var)\s+([a-zA-Z_$][0-9a-zA-Z_$]*)|function\s+([a-zA-Z_$][0-9a-zA-Z_$]*))"
                ).unwrap();
                if let Some(caps) = re.captures(line) {
                    let variable_name = caps.get(1).or_else(|| caps.get(2)).unwrap().as_str();
                    if variable_name.len() < 5 {
                        return context.end_of_handle(Some(&format!("A valtozoneveknek legalabb 5 karakter hosszunak kell lennie. Hibas valtozonev: {}", variable_name)));
                    }
                    if contains_number_or_hungarian_letter(variable_name) {
                        return context.end_of_handle(Some(&format!("A valtozonev szamot vagy ekezetes karaktert tartalmaz, ami rontja az olvashatosagot. Hibas valtozonev: {}", variable_name)));
                    }
                }
            }
        }
        context.end_of_handle(None)
    }

    fn success_message(&self) -> String {
        format!("Vatlozonevek rendben")
    }

    fn title(&self) -> String {
        format!("Valtozonevek ellenorzese...")
    }
}

fn contains_number_or_hungarian_letter(s: &str) -> bool {
    let hungarian_special_chars = [
        'á', 'é', 'í', 'ó', 'ö', 'ő', 'ú', 'ü', 'ű', 'Á', 'É', 'Í', 'Ó', 'Ö', 'Ő', 'Ú', 'Ü', 'Ű',
    ];

    s.chars()
        .any(|c| c.is_numeric() || hungarian_special_chars.contains(&c))
}
