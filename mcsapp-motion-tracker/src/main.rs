#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
extern crate mcslib_common;

mod networks;

use mcslib_common::config::ConfigLoader;
use mcslib_common::init_log;
use mcslib_common::once_cell::sync::Lazy;
use mcslib_common::types::{JsonSerializable, TrackersServerConfig};
use std::io::Result as IOResult;
use std::panic::set_hook;

const APP_CONFIG_PATH: &str = "app.config";

static APP_CONFIG: Lazy<TrackersServerConfig> =
    Lazy::new(|| TrackersServerConfig::load_config(APP_CONFIG_PATH).unwrap());

fn init_logging() {
    init_log(true);
    set_hook(Box::new(|panic_info| {
        error!("ABORT! {}", panic_info);
    }));
}

fn main() -> IOResult<()> {
    init_logging();
    debug!("{}", APP_CONFIG.to_json());
    Ok(())
}
