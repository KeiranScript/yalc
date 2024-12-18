#[macro_export]
macro_rules! info {
    ($msg:expr $(, $display:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display = $crate::macros::get_display(logger.debug $(, $display)?);
        logger.log($crate::logger::LogLevel::Info, &format!($msg), display);
    }};
}

#[macro_export]
macro_rules! warn {
    ($msg:expr $(, $display:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display = $crate::macros::get_display(true $(, $display)?);
        logger.log($crate::logger::LogLevel::Warn, &format!($msg), display);
    }};
}

#[macro_export]
macro_rules! error {
    ($msg:expr $(, $display:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display = $crate::macros::get_display(true $(, $display)?);
        logger.log($crate::logger::LogLevel::Error, &format!($msg), display);
    }};
}

#[macro_export]
macro_rules! success {
    ($msg:expr $(, $display:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display = $crate::macros::get_display(true $(, $display)?);
        logger.log($crate::logger::LogLevel::Success, &format!($msg), display);
    }};
}

pub fn get_display(global: bool, display: Option<bool>) -> bool {
    display.unwrap_or(global)
}
