#[macro_export]
macro_rules! info {
    ($msg:expr $(, $disp_val:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display = $crate::get_display(logger.is_debug(), Some($disp_val.unwrap_or(true)));
        logger.log($crate::logger::LogLevel::Info, &format!($msg), display);
    }};
}

#[macro_export]
macro_rules! warn {
    ($msg:expr $(, $disp_val:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display = $crate::get_display(logger.is_debug(), Some($disp_val.unwrap_or(true)));
        logger.log($crate::logger::LogLevel::Warn, &format!($msg), display);
    }};
}

#[macro_export]
macro_rules! error {
    ($msg:expr $(, $disp_val:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display = $crate::get_display(logger.is_debug(), Some($disp_val.unwrap_or(true)));
        logger.log($crate::logger::LogLevel::Error, &format!($msg), display);
    }};
}

#[macro_export]
macro_rules! success {
    ($msg:expr $(, $disp_val:expr)?) => {{
        let logger = $crate::logger::get_logger();
        let logger = logger.lock().unwrap();
        let display = $crate::get_display(logger.is_debug(), Some($disp_val.unwrap_or(true)));
        logger.log($crate::logger::LogLevel::Success, &format!($msg), display);
    }};
}

pub fn get_display(global: bool, display: Option<bool>) -> bool {
    display.unwrap_or(global)
}
