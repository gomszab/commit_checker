use commit_checker_core::{api::commit_checker_facade, message_handler::MessageOutput};

fn main() {
    let file_name = "something.js";
//     let file_contents = r#"
//     class {
//     /**
//      * @type {HTMLDivElement}
//      */
//     #errorDiv;
//     /**
//      * @type {HTMLInputElement}
//      */
//     #input;
//     /**
//      * @type {string}
//      */
//     #name;

//     /**
//      * @returns {string}
//      */
//     get name(){
//         return this.#name;
//     }

//     /**
//      * @returns {string}
//      */
//     get value(){
//         return this.#input.value;
//     }

//     set value(newVal){
//         this.#input.value = newVal;
//     }

//     validate(){
//         let result = true;
//         if(this.#input.value == ''){
//             this.#errorDiv.innerText = 'Kötelező'
//             result = false;
//         }else{
//             this.#errorDiv.innerText = '';
//         }
//         return result;
//     }
// }
//     "#;
let file_contents = r#"
/**
 * @type {string}
 */
const appleb = "apple" // variable declaration marked line above: no description
appleb.replace('a', ''); // replace
    "#;
    let mut adapter = ConsoleAdapter::new();
    let mut facade = commit_checker_facade::CommitCheckerFacade::build(&mut adapter);
    facade.analyze(file_name, file_contents);
}

struct ConsoleAdapter{

}

impl ConsoleAdapter{
    fn new()-> Self{
        Self {}
    }
}

impl MessageOutput for ConsoleAdapter{
    fn push(&mut self, message: commit_checker_core::message_handler::LocalizedMessage) {
        let typ = message.typ;
        match typ {
            commit_checker_core::message_handler::MessageType::ValidationError => {
                let details = message.details.to_string();
                let meta = &message.meta.unwrap();
                let params = &meta.params.clone().unwrap();
                let row = &meta.row.unwrap();
                let column_start = &meta.column_start.unwrap();
                let column_end = &meta.column_end.unwrap();
                let details = details.replace("%{line}", &row.to_string());
                println!("{}\n{}", details, format_args!("{}{}",
                            " ".repeat(*column_start),
                            "^".repeat(*column_end-*column_start)))
            },
            commit_checker_core::message_handler::MessageType::Info => {
                println!("[Info] {}", message.details)
            },
            commit_checker_core::message_handler::MessageType::Success => {
                println!("[Success] {} ", message.details)
            },
            commit_checker_core::message_handler::MessageType::Error => {
                println!("[Error] ")
            }
        }
    }
}