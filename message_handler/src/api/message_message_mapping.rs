use rust_i18n::t;

use crate::{IncommingMessage, MessageOutput, message_handler::{LocalizedMessage, MessageContext, MessageType, MetaInfo}};

pub fn handling_message(message: IncommingMessage, ctx: Option<&MessageContext>, handler: &dyn MessageOutput){
    let file_name = if let Some(MessageContext{ file_name}) = &ctx {
            Some(file_name.to_string())
        }else{
            None
        };
        let localized = match message {
            IncommingMessage::Error { code } => LocalizedMessage {
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
                row,
                column_start,
                column_end,
                params,
            } => LocalizedMessage {
                title: code.to_string(),
                details: render(t!(code).to_string(), &params),
                typ: MessageType::ValidationError,
                meta: Some(MetaInfo {
                    file_name,
                    row: Some(row),
                    column_start: Some(column_start),
                    column_end: Some(column_end),
                    params: Some(params),
                }),
            },
        };
        
        
        handler.push(ctx, localized);
        
}

fn render(details: String, params: &Vec<(String, String)>) -> String {
    let mut text = details;
    for (k, v) in params {
        text = text.replace(&format!("%{{{}}}", k), &v);
    }
    text
}