use crate::types::{BaseStationsConfig, JsonSerializable, TrackersConfig, TrackersServerConfig};
use std::fs::{read_to_string, write};
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};
use std::path::Path;

pub trait ConfigLoader<T: for<'a> JsonSerializable<'a, T> + Default> {
    fn init_default(config_path: &str) -> IOResult<()> {
        let path = Path::new(config_path);
        let config_exists = path.exists();

        if config_exists {
            return Err(IOError::new(
                IOErrorKind::AlreadyExists,
                format!("Config already exist in \"{}\"", config_path),
            ));
        }

        let default_config = T::default().to_json();
        write(path, default_config)
    }

    fn load_config(config_path: &str) -> IOResult<T> {
        let path = Path::new(config_path);
        let config_exists = path.exists();

        if !config_exists {
            return Err(IOError::new(
                IOErrorKind::AddrNotAvailable,
                format!("Cannot find \"{}\"", config_path),
            ));
        }

        let config_json = read_to_string(path)?;
        let config_data = T::from_json(&config_json)?;

        Ok(config_data)
    }
}

impl<'a> ConfigLoader<BaseStationsConfig> for BaseStationsConfig {}
impl<'a> ConfigLoader<TrackersConfig> for TrackersConfig {}
impl<'a> ConfigLoader<TrackersServerConfig> for TrackersServerConfig {}

pub fn init_configs(
    base_stations_config_path: &str,
    trackers_config_path: &str,
    tracker_server_config_path: &str,
) -> IOResult<()> {
    BaseStationsConfig::init_default(base_stations_config_path)?;
    TrackersConfig::init_default(trackers_config_path)?;
    TrackersServerConfig::init_default(tracker_server_config_path)
}

pub fn load_configs(
    base_stations_config_path: &str,
    trackers_config_path: &str,
    tracker_server_config_path: &str,
) -> IOResult<(BaseStationsConfig, TrackersConfig, TrackersServerConfig)> {
    let base_stations_config = BaseStationsConfig::load_config(base_stations_config_path)?;
    let trackers_config = TrackersConfig::load_config(trackers_config_path)?;
    let trackers_server_config = TrackersServerConfig::load_config(tracker_server_config_path)?;

    Ok((base_stations_config, trackers_config, trackers_server_config))
}
