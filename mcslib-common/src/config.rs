use crate::types::{JsonSerializable, TrackersServerConfig};
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

impl<'a> ConfigLoader<TrackersServerConfig> for TrackersServerConfig {}
