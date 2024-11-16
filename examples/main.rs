use yalc::{error, info, success, warn};

fn main() {
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");
    success!("This is a success message");
}
