#[macro_export]
macro_rules! rule_break_message {
    ($handler:expr, $code:expr, $file_name:expr, $($name:ident = $value:expr),+ $(,)?) => {{
        let params = vec![
            $(
                (stringify!($name).to_string(), ($value).to_string())
            ),+
        ];

        $handler.put_message(
            commit_checker_message_handler::Message::BreakingRule {
                code: $code,
                file_name: ($file_name).to_string(),
                params,
            }
        );
    }};
}

#[macro_export]
macro_rules! error_message {
    ($handler:expr, $code:expr) => {{
        $handler.put_message(commit_checker_message_handler::Message::Error {
            code: $code,
            param: None,
        });
    }};

    ($handler:expr, $code:expr, $param_name:ident = $param_value:expr) => {{
        use commit_checker_message_handler::MessageHandlerApi;
        $handler.put_message(commit_checker_message_handler::Message::Error {
            code: $code,
            param: Some(($param_value).to_string()),
        });
    }};
}

#[macro_export]
macro_rules! title_message {
    ($handler:expr, $code:expr) => {{
        $handler.put_message(commit_checker_message_handler::Message::Success { code: $code });
    }};
}

#[macro_export]
macro_rules! success_message {
    ($handler:expr, $code:expr) => {{
        $handler.put_message(commit_checker_message_handler::Message::Title { code: $code });
    }};
}
