use yalc::{error, info, init_logger, set_debug, success, warn};

#[test]
fn test_info_logging() {
    init_logger(true);
    info!("This is an info message");
    info!("This info message will not be displayed", false);
    set_debug(false);
    info!("This info message will be displayed", true);
}

#[test]
fn test_warn_logging() {
    init_logger(true);
    warn!("This is a warning message");
    warn!("This warning message will not be displayed", false);
    set_debug(false);
    warn!("This warning message will be displayed", true);
}

#[test]
fn test_error_logging() {
    init_logger(true);
    error!("This is an error message");
    error!("This error message will not be displayed", false);
    set_debug(false);
    error!("This error message will be displayed", true);
}

#[test]
fn test_success_logging() {
    init_logger(true);
    success!("This is a success message");
    success!("This success message will not be displayed", false);
    set_debug(false);
    success!("This success message will be displayed", true);
}
