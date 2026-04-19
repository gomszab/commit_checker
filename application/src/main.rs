use commit_checker_core::api::commit_checker_facade;

fn main(){
    let file_name = "something.js";
    let file_contents = r#"
    class FormField{
    /**
     * @type {HTMLDivElement}
     */
    #errorDiv;
    /**
     * @type {HTMLInputElement}
     */
    #input;
    /**
     * @type {string}
     */
    #name;

    /**
     * @returns {string}
     */
    get name(){
        return this.#name;
    }

    /**
     * @returns {string}
     */
    get value(){
        return this.#input.value;
    }

    set value(newVal){
        this.#input.value = newVal;
    }

    validate(){
        let result = true;
        if(this.#input.value == ''){
            this.#errorDiv.innerText = 'Kötelező'
            result = false;
        }else{
            this.#errorDiv.innerText = '';
        }
        return result;
    }
}
    "#;
    let mut facade = commit_checker_facade::CommitCheckerFacade::build();
    facade.analyze(file_name, file_contents);
}
