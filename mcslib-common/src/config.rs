use crate::types::{BaseStationsConfig, JsonSerializable, TrackersConfig};
use std::fs::{read_to_string, write};
use std::io::{Error as IOError, ErrorKind as IOErrorKind, Result as IOResult};
use std::path::Path;

pub fn init_default_stations_config(config_path: &str) -> IOResult<()> {
    let path = Path::new(config_path);
    let config_exist = path.exists();

    if config_exist {
        return Err(IOError::new(
            IOErrorKind::AlreadyExists,
            format!("Config already exist in \"{}\"", config_path),
        ));
    }

    let default_stations_config = BaseStationsConfig::default().to_json();
    write(path, default_stations_config)
}

pub fn load_stations_config(config_path: &str) -> IOResult<BaseStationsConfig> {
    let path = Path::new(config_path);
    let config_exist = path.exists();

    if !config_exist {
        return Err(IOError::new(
            IOErrorKind::AddrNotAvailable,
            format!("Cannot find \"{}\"", config_path),
        ));
    }

    let config_json = read_to_string(path)?;

    match BaseStationsConfig::from_json(&config_json) {
        Ok(config) => Ok(config),
        Err(_) => Err(IOError::new(
            IOErrorKind::InvalidData,
            format!("Cannot parse \"{}\" to BaseStationsConfig", config_path),
        )),
    }
}

pub fn init_default_tracker_config(config_path: &str) -> IOResult<()> {
    let path = Path::new(config_path);
    let config_exist = path.exists();

    if config_exist {
        return Err(IOError::new(
            IOErrorKind::AlreadyExists,
            format!("Config already exist in \"{}\"", config_path),
        ));
    }

    let default_trackers_config = TrackersConfig::default().to_json();
    write(path, default_trackers_config)
}

pub fn load_trackers_config(config_path: &str) -> IOResult<TrackersConfig> {
    let path = Path::new(config_path);
    let config_exist = path.exists();

    if !config_exist {
        return Err(IOError::new(
            IOErrorKind::AddrNotAvailable,
            format!("Cannot find \"{}\"", config_path),
        ));
    }

    let config_json = read_to_string(path)?;

    match TrackersConfig::from_json(&config_json) {
        Ok(config) => Ok(config),
        Err(_) => Err(IOError::new(
            IOErrorKind::InvalidData,
            format!("Cannot parse \"{}\" to TrackersConfig", config_path),
        )),
    }
}

pub fn init_configs(base_stations_config_path: &str, trackers_config_path: &str) -> IOResult<()> {
    init_default_stations_config(base_stations_config_path)?;
    init_default_tracker_config(trackers_config_path)
}

pub fn load_configs(
    base_stations_config_path: &str,
    trackers_config_path: &str,
) -> IOResult<(BaseStationsConfig, TrackersConfig)> {
    let base_stations_config = load_stations_config(base_stations_config_path)?;
    let trackers_config = load_trackers_config(trackers_config_path)?;

    Ok((base_stations_config, trackers_config))
}
