use colored::Colorize;
use crate::api::FileContext;

pub struct ErrorHandler<'a>{
    file_contexts: Vec<&'a mut FileContext<'a>>,
    errored_files: Vec<(&'a str, Vec<String>)>,
    ok_files: Vec<&'a str>
}
impl<'a> ErrorHandler<'a> {
    pub fn new() -> Self{
        let err_handler = ErrorHandler{
            file_contexts: Vec::new(),
            errored_files: Vec::new(),
            ok_files: Vec::new()
        };
        err_handler
    }

    pub fn subscribe(&mut self, file_context: &'a mut FileContext<'a>){
        self.file_contexts.push(file_context);
    }

    pub fn show_errors(&self){

    }

    pub fn summa(&self){

    }

    fn gather_handler_results(&self){

    }

    pub fn print_errors(errors: Vec<String>) {
        for error in errors {
            eprintln!("{}", error.red());
        }
    }

    pub fn print_error(message: String){
        eprintln!("{}", message.red());
    }
}