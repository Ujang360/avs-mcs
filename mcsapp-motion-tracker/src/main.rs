#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[macro_use]
extern crate mcslib_common;

mod networks;

use mcslib_common::config::{init_configs, load_configs};
use mcslib_common::init_log;
use std::io::Result as IOResult;

const STATIONS_CONFIG_PATH: &str = "base-stations.config";
const TRACKERS_CONFIG_PATH: &str = "motion-trackers.config";
const APP_CONFIG_PATH: &str = "app.config";

fn main() -> IOResult<()> {
    init_log(true);
    let _ = init_configs(STATIONS_CONFIG_PATH, TRACKERS_CONFIG_PATH, APP_CONFIG_PATH);
    let (base_stations_config, motion_trackers_config, app_config) =
        load_configs(STATIONS_CONFIG_PATH, TRACKERS_CONFIG_PATH, APP_CONFIG_PATH)?;
    debug!("{}", base_stations_config);
    debug!("{}", motion_trackers_config);
    debug!("{}", app_config);
    Ok(())
}
