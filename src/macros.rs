#[macro_export]
macro_rules! info {
    ($msg:expr $(, $disp:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display_value = $crate::macros::get_display(logger.debug, $disp);
        logger.log(
            $crate::logger::LogLevel::Info,
            &format!($msg),
            display_value,
        );
    }};
}

#[macro_export]
macro_rules! warn {
    ($msg:expr $(, $disp:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display_value = $crate::macros::get_display(logger.debug, $disp);
        logger.log(
            $crate::logger::LogLevel::Warn,
            &format!($msg),
            display_value,
        );
    }};
}

#[macro_export]
macro_rules! error {
    ($msg:expr $(, $disp:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display_value = $crate::macros::get_display(logger.debug, $disp);
        logger.log(
            $crate::logger::LogLevel::Error,
            &format!($msg),
            display_value,
        );
    }};
}

#[macro_export]
macro_rules! success {
    ($msg:expr $(, $disp:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display_value = $crate::macros::get_display(logger.debug, $disp);
        logger.log(
            $crate::logger::LogLevel::Success,
            &format!($msg),
            display_value,
        );
    }};
}

pub fn get_display(global: bool, display: Option<bool>) -> bool {
    display.unwrap_or(global)
}
