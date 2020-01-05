pub extern crate bytes;
pub extern crate chrono;
pub extern crate env_logger;
pub extern crate log;
pub extern crate serialport;

pub use log::{debug, error, info, trace, warn};
pub use uuid::Uuid;

use chrono::Utc;
use env_logger::builder as log_builder;
use std::env::set_var as set_env_var;

pub mod config;
pub mod types;

pub fn exit_with_error(error_message: &str) -> ! {
    error!("{}", error_message);
    panic!("{}", error_message)
}

pub fn init_log(debug_mode: bool) {
    set_env_var("RUST_LOG", if debug_mode { "debug" } else { "info" });
    log_builder()
        .default_format()
        .format_timestamp_nanos()
        .format_indent(Some(4))
        .init();
}

pub fn get_new_uuidv4() -> Uuid {
    Uuid::new_v4()
}

pub fn get_timestamp_nanos() -> i64 {
    Utc::now().timestamp_nanos()
}

pub fn get_timestamp_millis() -> i64 {
    Utc::now().timestamp_millis()
}

pub fn get_timestamp() -> i64 {
    Utc::now().timestamp()
}

pub fn get_rfc3339_now() -> String {
    Utc::now().to_rfc3339()
}
