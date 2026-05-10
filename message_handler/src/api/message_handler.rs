#[derive(Clone)]
pub struct MessageContext {
    pub file_name: String,
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

pub enum MessageType {
    Error,
    ValidationError,
    Success,
    Info,
}

pub enum IncommingMessage {
    RuleStart {
        code: &'static str,
    },
    Error {
        code: &'static str,
    },
    ValidationError {
        code: &'static str,
        row: usize,
        column_start: usize,
        column_end: usize,
        params: Vec<(String, String)>,
    },
    RuleSuccess {
        code: &'static str,
    },
}