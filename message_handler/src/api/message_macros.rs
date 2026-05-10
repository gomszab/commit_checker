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
    ($code:expr, $row:expr, $column_start:expr, $column_end:expr, $($name:ident = $value:expr),+ $(,)? ) => {
        let params = vec![
            $(
                (stringify!($name).to_string(), ($value).to_string())
            ),+
        ];
        commit_checker_message_handler::handle_with_context(commit_checker_message_handler::IncommingMessage::ValidationError {
                code: $code,
                row: $row,
                column_start: $column_start,
                column_end: $column_end,
                params
        })
    };

    ($code:expr, $row:expr, $column_start:expr, $column_end:expr $(,)?) => {

        let params: Vec<(String, String)> = Vec::new();
        commit_checker_message_handler::handle_with_context(commit_checker_message_handler::IncommingMessage::ValidationError {
            code: $code,
            row: $row,
            column_start: $column_start,
            column_end: $column_end,
            params
        })
    };
}

#[macro_export]
macro_rules! software_error {
    ($code:expr) => {
        commit_checker_message_handler::handle_with_context(commit_checker_message_handler::IncommingMessage::Error {
            code: $code
        })
    };
}

#[macro_export]
macro_rules! info_message {
    ($code:expr) => {
        commit_checker_message_handler::handle_with_context(commit_checker_message_handler::IncommingMessage::RuleStart { code: $code })
    };
}

#[macro_export]
macro_rules! rule_success_message {
    ($code:expr) => {
        commit_checker_message_handler::handle_with_context(commit_checker_message_handler::IncommingMessage::RuleSuccess { code: $code })
    };
}

#[macro_export]
macro_rules! init_message_handler {
    ($language:expr, $factory:expr) => {
        commit_checker_message_handler::message_impl::init_message_api($factory, Some($language))
    };
    ($factory:expr) => {
        commit_checker_message_handler::message_impl::init_message_api($factory, None)
    };
}
