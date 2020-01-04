pub extern crate env_logger;
pub extern crate log;

pub use log::{debug, error, info, trace, warn};

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
