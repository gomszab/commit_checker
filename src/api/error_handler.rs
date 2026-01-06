use colored::Colorize;

pub struct ErrorHandler{}
impl ErrorHandler {
    pub fn print_errors(errors: Vec<String>) {
        for error in errors {
            eprintln!("{}", error.red());
        }
    }

    pub fn print_error(message: String){
        eprintln!("{}", message.red());
    }
}