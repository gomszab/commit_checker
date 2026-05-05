/// Reports a validation error.
///
/// Signature-like:
/// ```text
/// validation_error!(handler, code, file_name, row, col_start, col_end [, name = value, ...])
/// ```
///
/// * `handler` – object handling the message
/// * `code` – validation error code, e.g. "C01"
/// * `file_name` – file where the error occurred
/// * `row`, `col_start`, `col_end` – 1-based position
/// * `name = value` – optional parameters included in `params`
#[macro_export]
macro_rules! validation_error {
    ($handler:expr, $code:expr, $file_name:expr, $row:expr, $column_start:expr, $column_end:expr, $($name:ident = $value:expr),+ $(,)? ) => {
        use commit_checker_message_handler::{
    MessageHandlerApi};
        let params = vec![
            $(
                (stringify!($name).to_string(), ($value).to_string())
            ),+
        ];
        $handler.handle(commit_checker_message_handler::IncommingMessage::ValidationError {
                code: $code,
                file_name: ($file_name).to_string(),
                row: $row,
                column_start: $column_start,
                column_end: $column_end,
                params
        })
    };

    ($handler:expr, $code:expr, $file_name:expr, $row:expr, $column_start:expr, $column_end:expr $(,)?) => {
        use commit_checker_message_handler::{
    MessageHandlerApi};
        let params: Vec<(String, String)> = Vec::new();
        $handler.handle(commit_checker_message_handler::IncommingMessage::ValidationError {
            code: $code,
            file_name: ($file_name).to_string(),
            row: $row,
            column_start: $column_start,
            column_end: $column_end,
            params
        })
    };
}

#[macro_export]
macro_rules! software_error {
    ($handler:expr, $code:expr, $file_name:expr) => {
        use commit_checker_message_handler::MessageHandlerApi;
        $handler.handle(commit_checker_message_handler::IncommingMessage::Error {
            code: $code,
            file_name: $file_name,
        })
    };
}

#[macro_export]
macro_rules! info_message {
    ($handler:expr, $code:expr) => {
        use commit_checker_message_handler::MessageHandlerApi;
        $handler.handle(commit_checker_message_handler::IncommingMessage::RuleStart { code: $code })
    };
}

#[macro_export]
macro_rules! rule_success_message {
    ($handler:expr, $code:expr) => {
        use commit_checker_message_handler::MessageHandlerApi;
        $handler
            .handle(commit_checker_message_handler::IncommingMessage::RuleSuccess { code: $code })
    };
}
