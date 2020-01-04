use serde::{Deserialize, Serialize};
use serde_json::{from_str as from_string, to_string, to_string_pretty as to_json, Error as JsonError};
use std::fmt::{Display, Formatter, Result as FormatterResult};
use std::net::SocketAddrV4;

const JSON_MAPPING_ERROR_MESSAGE: &str = "Possible JSON mapping failure!";

pub trait JsonSerializable<'a, T = Self>
where
    Self: Deserialize<'a> + Serialize,
{
    fn from_json(json_string: &'a str) -> Result<Self, JsonError> {
        from_string::<'a, Self>(json_string)
    }

    fn to_json(&self) -> String {
        to_json(self).expect(JSON_MAPPING_ERROR_MESSAGE)
    }

    fn to_string(&self) -> String {
        to_string(self).expect(JSON_MAPPING_ERROR_MESSAGE)
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SafePoint2D {
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SafePoint3D {
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "Z")]
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SafeEulerAngles {
    #[serde(rename = "Roll")]
    pub roll: f64,
    #[serde(rename = "Pitch")]
    pub pitch: f64,
    #[serde(rename = "Yaw")]
    pub yaw: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BaseStations {
    #[serde(rename = "Position")]
    pub position: SafePoint3D,
    #[serde(rename = "Rotation")]
    pub rotation: SafeEulerAngles,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BaseStationsConfig {
    #[serde(rename = "StationA")]
    pub station_a: BaseStations,
    #[serde(rename = "StationB")]
    pub station_b: BaseStations,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TrackerEndpoint {
    #[serde(rename = "TrackerName")]
    pub tracker_name: String,
    #[serde(rename = "Endpoint")]
    pub endpoint: SocketAddrV4,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TrackersConfig(Vec<TrackerEndpoint>);

impl JsonSerializable<'_> for SafePoint2D {}
impl JsonSerializable<'_> for SafePoint3D {}
impl JsonSerializable<'_> for SafeEulerAngles {}
impl JsonSerializable<'_> for BaseStations {}
impl JsonSerializable<'_> for BaseStationsConfig {}
impl JsonSerializable<'_> for TrackerEndpoint {}
impl JsonSerializable<'_> for TrackersConfig {}

impl Display for SafePoint2D {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for SafePoint3D {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for SafeEulerAngles {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for BaseStations {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for BaseStationsConfig {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for TrackerEndpoint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for TrackersConfig {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Default for SafePoint2D {
    fn default() -> SafePoint2D {
        SafePoint2D { x: 0.0, y: 0.0 }
    }
}

impl Default for SafePoint3D {
    fn default() -> SafePoint3D {
        SafePoint3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl Default for SafeEulerAngles {
    fn default() -> SafeEulerAngles {
        SafeEulerAngles {
            pitch: 0.0,
            roll: 0.0,
            yaw: 0.0,
        }
    }
}

impl Default for BaseStations {
    fn default() -> BaseStations {
        BaseStations {
            position: Default::default(),
            rotation: Default::default(),
        }
    }
}

impl Default for BaseStationsConfig {
    fn default() -> BaseStationsConfig {
        BaseStationsConfig {
            station_a: Default::default(),
            station_b: Default::default(),
        }
    }
}

impl Default for TrackerEndpoint {
    fn default() -> TrackerEndpoint {
        TrackerEndpoint {
            tracker_name: "".into(),
            endpoint: "127.0.0.1:5000".parse().unwrap(),
        }
    }
}

impl Default for TrackersConfig {
    fn default() -> TrackersConfig {
        TrackersConfig(vec![TrackerEndpoint::default()])
    }
}
