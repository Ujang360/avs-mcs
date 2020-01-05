use crate::serialport::{
    DataBits as SPDataBits, FlowControl as SPFlowControl, Parity as SPParity, SerialPortSettings as SPSettings,
    StopBits as SPStopBits,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str as from_string, to_string, to_string_pretty as to_json, Error as JsonError};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt::{Display, Formatter, Result as FormatterResult};
use std::net::SocketAddrV4;
use std::time::Duration;

pub type SerialPortName = String;

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

#[repr(u32)]
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug, Copy)]
pub enum Baud {
    B4800 = 4_800,
    B9600 = 9_600,
    B19200 = 19_200,
    B38400 = 38_400,
    B57600 = 57_600,
    B115200 = 115_200,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug, Copy)]
pub enum DataBits {
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
pub enum FlowControl {
    None,
    Software,
    Hardware,
}

#[derive(Deserialize, Serialize, Clone, Debug, Copy)]
pub enum Parity {
    None,
    Odd,
    Even,
}

#[repr(u8)]
#[derive(Deserialize_repr, Serialize_repr, Clone, Debug, Copy)]
pub enum StopBits {
    One = 1,
    Two = 2,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SerialPortSettings {
    pub name: SerialPortName,
    pub baud_rate: Baud,
    pub data_bits: DataBits,
    pub flow_control: FlowControl,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub timeout: Duration,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ServerType {
    SerialPort(Vec<SerialPortSettings>),
    UDP(SocketAddrV4),
    TCP(SocketAddrV4),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum TrackerCommunication {
    SerialPort(SerialPortSettings),
    UDP(SocketAddrV4),
    TCP(SocketAddrV4),
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
    #[serde(rename = "TrackerCommunication")]
    pub tracker_communication: TrackerCommunication,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TrackersConfig(Vec<TrackerEndpoint>);

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TrackersServerConfig {
    #[serde(rename = "ServerName")]
    pub name: String,
    #[serde(rename = "IGServerType")]
    pub ig_server_type: ServerType,
}

impl Into<SPDataBits> for DataBits {
    fn into(self) -> SPDataBits {
        match self {
            DataBits::Five => SPDataBits::Five,
            DataBits::Six => SPDataBits::Six,
            DataBits::Seven => SPDataBits::Seven,
            DataBits::Eight => SPDataBits::Eight,
        }
    }
}

impl Into<SPFlowControl> for FlowControl {
    fn into(self) -> SPFlowControl {
        match self {
            FlowControl::None => SPFlowControl::None,
            FlowControl::Software => SPFlowControl::Software,
            FlowControl::Hardware => SPFlowControl::Hardware,
        }
    }
}

impl Into<SPParity> for Parity {
    fn into(self) -> SPParity {
        match self {
            Parity::None => SPParity::None,
            Parity::Odd => SPParity::Odd,
            Parity::Even => SPParity::Even,
        }
    }
}

impl Into<SPStopBits> for StopBits {
    fn into(self) -> SPStopBits {
        match self {
            StopBits::One => SPStopBits::One,
            StopBits::Two => SPStopBits::Two,
        }
    }
}

impl SerialPortSettings {
    pub fn get_sp_settings(&self) -> (SerialPortName, SPSettings) {
        (
            self.name.clone(),
            SPSettings {
                baud_rate: self.baud_rate as u32,
                data_bits: self.data_bits.into(),
                flow_control: self.flow_control.into(),
                parity: self.parity.into(),
                stop_bits: self.stop_bits.into(),
                timeout: self.timeout,
            },
        )
    }
}

impl JsonSerializable<'_> for Baud {}
impl JsonSerializable<'_> for DataBits {}
impl JsonSerializable<'_> for FlowControl {}
impl JsonSerializable<'_> for Parity {}
impl JsonSerializable<'_> for StopBits {}
impl JsonSerializable<'_> for SerialPortSettings {}
impl JsonSerializable<'_> for ServerType {}
impl JsonSerializable<'_> for SafePoint2D {}
impl JsonSerializable<'_> for SafePoint3D {}
impl JsonSerializable<'_> for SafeEulerAngles {}
impl JsonSerializable<'_> for BaseStations {}
impl JsonSerializable<'_> for BaseStationsConfig {}
impl JsonSerializable<'_> for TrackerCommunication {}
impl JsonSerializable<'_> for TrackerEndpoint {}
impl JsonSerializable<'_> for TrackersConfig {}
impl JsonSerializable<'_> for TrackersServerConfig {}

impl Display for Baud {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for DataBits {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for FlowControl {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for Parity {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for StopBits {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for SerialPortSettings {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Display for ServerType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

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

impl Display for TrackerCommunication {
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

impl Display for TrackersServerConfig {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatterResult {
        write!(formatter, "{}", self.to_json())
    }
}

impl Default for Baud {
    fn default() -> Baud {
        Baud::B115200
    }
}

impl Default for DataBits {
    fn default() -> DataBits {
        DataBits::Eight
    }
}

impl Default for FlowControl {
    fn default() -> FlowControl {
        FlowControl::None
    }
}

impl Default for Parity {
    fn default() -> Parity {
        Parity::None
    }
}

impl Default for StopBits {
    fn default() -> StopBits {
        StopBits::Two
    }
}

impl Default for SerialPortSettings {
    fn default() -> SerialPortSettings {
        SerialPortSettings {
            name: "/dev/ttyS0".into(),
            baud_rate: Default::default(),
            data_bits: Default::default(),
            parity: Default::default(),
            stop_bits: Default::default(),
            flow_control: Default::default(),
            timeout: Duration::from_secs(1),
        }
    }
}

impl Default for ServerType {
    fn default() -> ServerType {
        ServerType::TCP("127.0.0.1:4000".parse().unwrap())
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

impl Default for TrackerCommunication {
    fn default() -> TrackerCommunication {
        TrackerCommunication::UDP("127.0.0.1:5000".parse().unwrap())
    }
}

impl Default for TrackerEndpoint {
    fn default() -> TrackerEndpoint {
        TrackerEndpoint {
            tracker_name: Default::default(),
            tracker_communication: Default::default(),
        }
    }
}

impl Default for TrackersConfig {
    fn default() -> TrackersConfig {
        TrackersConfig(vec![TrackerEndpoint::default()])
    }
}

impl Default for TrackersServerConfig {
    fn default() -> TrackersServerConfig {
        TrackersServerConfig {
            name: "Motion Tracker Server".into(),
            ig_server_type: Default::default(),
        }
    }
}
