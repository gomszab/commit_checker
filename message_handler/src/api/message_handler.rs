use crate::t;

pub trait MessageHandlerApi {
    fn handle(&mut self, message: IncommingMessage);
}

pub enum IncommingMessage {
    RuleStart {
        code: &'static str,
    },
    Error {
        code: &'static str,
        file_name: Option<String>,
    },
    ValidationError {
        code: &'static str,
        file_name: String,
        row: usize,
        column_start: usize,
        column_end: usize,
        params: Vec<(String, String)>,
    },
    RuleSuccess {
        code: &'static str,
    },
}

pub enum MessageType {
    Error,
    ValidationError,
    Success,
    Info,
}
pub struct MetaInfo {
    pub file_name: Option<String>,
    pub row: Option<usize>,
    pub column_start: Option<usize>,
    pub column_end: Option<usize>,
    pub params: Option<Vec<(String, String)>>,
}

pub struct LocalizedMessage {
    pub title: String,
    pub details: String,
    pub typ: MessageType,
    pub meta: Option<MetaInfo>,
}

pub trait MessageOutput {
    fn push(&mut self, message: LocalizedMessage);
}

pub struct MessageHandler<'a> {
    adapter: &'a mut dyn MessageOutput,
}

impl<'a> MessageHandler<'a> {
    pub fn build(adapter: &'a mut dyn MessageOutput) -> Self {
        rust_i18n::set_locale("hu");
        MessageHandler { adapter }
    }
}

impl<'a> MessageHandlerApi for MessageHandler<'a> {
    fn handle(&mut self, message: IncommingMessage) {
        let localized = match message {
            IncommingMessage::Error { code, file_name } => LocalizedMessage {
                typ: MessageType::Error,
                details: if let Some(file_name) = &file_name {
                    t!(code, file_name = file_name.to_string()).to_string()
                } else {
                    t!(code).to_string()
                },
                title: code.to_string(),
                meta: Some(MetaInfo {
                    file_name: file_name,
                    row: None,
                    column_start: None,
                    column_end: None,
                    params: None,
                }),
            },
            IncommingMessage::RuleStart { code } => LocalizedMessage {
                typ: MessageType::Info,
                title: code.to_string(),
                details: t!(code).to_string(),
                meta: None,
            },
            IncommingMessage::RuleSuccess { code } => LocalizedMessage {
                title: code.to_string(),
                details: t!(code).to_string(),
                typ: MessageType::Success,
                meta: None,
            },
            IncommingMessage::ValidationError {
                code,
                file_name,
                row,
                column_start,
                column_end,
                params,
            } => LocalizedMessage {
                title: code.to_string(),
                details: render(t!(code).to_string(), &params),
                typ: MessageType::ValidationError,
                meta: Some(MetaInfo {
                    file_name: Some(file_name),
                    row: Some(row),
                    column_start: Some(column_start),
                    column_end: Some(column_end),
                    params: Some(params),
                }),
            },
        };
        self.adapter.push(localized);
    }
}

fn render(details: String, params: &Vec<(String, String)>) -> String {
    let mut text = details;
    for (k, v) in params {
        text = text.replace(&format!("%{{{}}}", k), &v);
    }
    text
}
