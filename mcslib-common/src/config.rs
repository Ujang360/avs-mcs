use crate::types::{BaseStationsConfig, JsonSerializable, TrackersConfig, TrackersServerConfig};
use std::fs::{read_to_string, write};
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};
use std::path::Path;

pub trait ConfigLoader<'a, T = Self>
where
    Self: JsonSerializable<'a> + Default,
{
    fn init_default(config_path: &str) -> IOResult<()> {
        let path = Path::new(config_path);
        let config_exists = path.exists();

        if config_exists {
            return Err(IOError::new(
                IOErrorKind::AlreadyExists,
                format!("Config already exist in \"{}\"", config_path),
            ));
        }

        let default_config = Self::default().to_json();
        write(path, default_config)
    }

    fn load_config_json(config_path: &str) -> IOResult<String> {
        let path = Path::new(config_path);
        let config_exists = path.exists();

        if !config_exists {
            return Err(IOError::new(
                IOErrorKind::AddrNotAvailable,
                format!("Cannot find \"{}\"", config_path),
            ));
        }

        read_to_string(path)
    }
}

impl<'a> ConfigLoader<'a> for BaseStationsConfig {}
impl<'a> ConfigLoader<'a> for TrackersConfig {}
impl<'a> ConfigLoader<'a> for TrackersServerConfig {}

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
    let base_stations_config_json = BaseStationsConfig::load_config_json(base_stations_config_path)?;
    let trackers_config_json = TrackersConfig::load_config_json(trackers_config_path)?;
    let trackers_server_config_json = TrackersServerConfig::load_config_json(tracker_server_config_path)?;
    let base_stations_config = BaseStationsConfig::from_json(&base_stations_config_json)?;
    let trackers_config = TrackersConfig::from_json(&trackers_config_json)?;
    let trackers_server_config = TrackersServerConfig::from_json(&trackers_server_config_json)?;

    Ok((base_stations_config, trackers_config, trackers_server_config))
}
