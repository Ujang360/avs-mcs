#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
extern crate mcslib_common;

mod networks;

use mcslib_common::config::{init_configs, load_configs};
use mcslib_common::init_log;
use std::io::Result as IOResult;
use std::panic::set_hook;

const STATIONS_CONFIG_PATH: &str = "base-stations.config";
const TRACKERS_CONFIG_PATH: &str = "motion-trackers.config";
const APP_CONFIG_PATH: &str = "app.config";

fn init_logging() {
    init_log(true);
    set_hook(Box::new(|panic_info| {
        error!("ABORT! {}", panic_info);
    }));
}

fn main() -> IOResult<()> {
    init_logging();
    let _ = init_configs(STATIONS_CONFIG_PATH, TRACKERS_CONFIG_PATH, APP_CONFIG_PATH);
    let (base_stations_config, motion_trackers_config, app_config) =
        load_configs(STATIONS_CONFIG_PATH, TRACKERS_CONFIG_PATH, APP_CONFIG_PATH)?;
    debug!("{}", base_stations_config);
    debug!("{}", motion_trackers_config);
    debug!("{}", app_config);
    Ok(())
}
