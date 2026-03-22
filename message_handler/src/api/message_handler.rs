use std::collections::HashMap;

use crate::t;

pub trait MessageHandlerApi {

    fn put_message(&mut self, message: Message);

    fn get_messages_ordered_by_file_name(&self) -> HashMap<String, Vec<String>>;

    fn get_message_by_file(&self, file_name: String) -> Vec<String>;
}

pub enum Message {
    Success {code: &'static str},
    Error {code: &'static str, param: Option<String>},
    BreakingRule {
        code: &'static str,
        file_name: String,
        params: Vec<(String, String)>
    }
}

struct MessageHandler {
    messages: Vec<Message>
}

impl MessageHandler {
    fn build() -> Self {
        MessageHandler { messages: Vec::new() }
    }
}

impl MessageHandlerApi for MessageHandler {
    fn put_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    fn get_messages_ordered_by_file_name(&self) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for msg in &self.messages{
            if let Message::BreakingRule {file_name, ..} = msg {
                let text: String = render_message(msg);
                map.entry(file_name.clone())
                .or_insert_with(Vec::new).push(text);
            }
        }
        map
    }

    fn get_message_by_file(&self, file_name: String) -> Vec<String> {
        todo!()
    }
}

fn render_message(message: &Message) -> String {
    match message {
         Message::Error { code, param }  => {
            todo!()
         },
         Message::Success { code } => {
            todo!()
         },
         Message::BreakingRule { code, file_name, params } => {
             let mut text = t!(*code).to_string();
             let mut full_param_list = params.clone();
             full_param_list.push(("file_name".to_string(), file_name.to_string()));
            for (k, v) in params {
                text = text.replace(&format!("{{{}}}", k), v);
            }
            text
         }
    }
}