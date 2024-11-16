use std::sync::{Arc, Mutex, Once};

pub enum LogLevel {
    Info,
    Warn,
    Error,
    Success,
}

pub struct Logger {
    debug: bool,
}

impl Logger {
    pub fn new(debug: bool) -> Self {
        Logger { debug }
    }

    pub fn log(&self, level: LogLevel, message: &str, display: bool) {
        if !display {
            return;
        }

        let color_code = match level {
            LogLevel::Info => "\x1b[34m",    // blue
            LogLevel::Warn => "\x1b[33m",    // yellow
            LogLevel::Error => "\x1b[31m",   // red
            LogLevel::Success => "\x1b[32m", // green
        };

        println!("{}{}\x1b[0m", color_code, message);
    }

    pub fn is_debug(&self) -> bool {
        self.debug
    }
}

static LOGGER: Once = Once::new();
static mut GLOBAL_LOGGER: Option<Arc<Mutex<Logger>>> = None;

pub fn init_logger(debug: bool) {
    LOGGER.call_once(|| unsafe {
        GLOBAL_LOGGER = Some(Arc::new(Mutex::new(Logger::new(debug))));
    });
}

pub fn get_logger() -> Arc<Mutex<Logger>> {
    unsafe { GLOBAL_LOGGER.clone().expect("Logger is not initialized") }
}

pub fn set_debug(debug: bool) {
    let logger = get_logger();
    let mut logger = logger.lock().unwrap();
    logger.debug = debug;
}
