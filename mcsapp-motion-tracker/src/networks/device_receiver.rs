use crossbeam_channel::{unbounded, Receiver, Sender};
use crossbeam_utils::thread::{scope, Scope, ScopedJoinHandle};
use mcslib_common::bytes::Bytes;
use mcslib_common::types::{SerialPortSettings as SPSettings, TrackerCommunication, TrackersConfig};
use std::collections::HashMap;
use std::mem;
use std::net::SocketAddrV4;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU8, AtomicUsize, Ordering};
use std::sync::Mutex;

pub type MotionTrackerDataReceiver = Receiver<Bytes>;

trait IOBroadcastReceiver<'a, T = Self>
where
    Self: Sized,
{
    fn new(trackers_config: &TrackersConfig) -> Result<Self, ()>;
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum IOBroadcastReceiverState {
    Stopped = 0,
    Starting = 1,
    Started = 2,
    Stopping = 3,
}

#[derive(Debug)]
pub struct IOBroadcastReceiverUDP<'a> {
    state: AtomicU8,
    is_stop_requested: AtomicBool,
    subscriber_count: AtomicUsize,
    receiving_counter: AtomicUsize,
    start_timestamp: AtomicI64,
    broadcaster_addresses: HashMap<String, SocketAddrV4>,
    pub_thread: Option<Vec<ScopedJoinHandle<'a, ()>>>,
    pubsub_channel: Mutex<Option<(Sender<Bytes>, MotionTrackerDataReceiver)>>,
}

#[derive(Debug)]
pub struct IOBroadcastReceiverTCP<'a> {
    state: AtomicU8,
    is_stop_requested: AtomicBool,
    subscriber_count: AtomicUsize,
    receiving_counter: AtomicUsize,
    start_timestamp: AtomicI64,
    broadcaster_addresses: HashMap<String, SocketAddrV4>,
    pub_thread: Option<Vec<ScopedJoinHandle<'a, ()>>>,
    pubsub_channel: Mutex<Option<(Sender<Bytes>, MotionTrackerDataReceiver)>>,
}

#[derive(Debug)]
pub struct IOBroadcastReceiverSerial<'a> {
    state: AtomicU8,
    is_stop_requested: AtomicBool,
    subscriber_count: AtomicUsize,
    receiving_counter: AtomicUsize,
    start_timestamp: AtomicI64,
    broadcaster_addresses: HashMap<String, SPSettings>,
    pub_thread: Option<Vec<ScopedJoinHandle<'a, ()>>>,
    pubsub_channel: Mutex<Option<(Sender<Bytes>, MotionTrackerDataReceiver)>>,
}

impl Into<u8> for IOBroadcastReceiverState {
    #[inline]
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for IOBroadcastReceiverState {
    #[inline]
    fn from(value: u8) -> IOBroadcastReceiverState {
        match value {
            0 => IOBroadcastReceiverState::Stopped,
            1 => IOBroadcastReceiverState::Starting,
            2 => IOBroadcastReceiverState::Started,
            3 => IOBroadcastReceiverState::Stopping,
            _ => panic!("Invalid state value!"),
        }
    }
}

impl PartialEq for IOBroadcastReceiverState {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl Eq for IOBroadcastReceiverState {}

impl<'a> IOBroadcastReceiver<'a> for IOBroadcastReceiverUDP<'a> {
    fn new(trackers_config: &TrackersConfig) -> Result<Self, ()> {
        let mut udp_trackers: HashMap<String, SocketAddrV4> = Default::default();

        for tracker_config in &trackers_config.0 {
            if let TrackerCommunication::UDP(address) = tracker_config.tracker_communication {
                udp_trackers.insert(tracker_config.tracker_name.clone(), address);
            }
        }

        if udp_trackers.is_empty() {
            return Err(());
        }

        Ok(IOBroadcastReceiverUDP {
            state: AtomicU8::new(IOBroadcastReceiverState::Stopped.into()),
            is_stop_requested: AtomicBool::new(false),
            subscriber_count: AtomicUsize::new(0),
            receiving_counter: AtomicUsize::new(0),
            start_timestamp: AtomicI64::new(0),
            broadcaster_addresses: udp_trackers,
            pub_thread: None,
            pubsub_channel: Mutex::new(None),
        })
    }
}

impl<'a> IOBroadcastReceiver<'a> for IOBroadcastReceiverTCP<'a> {
    fn new(trackers_config: &TrackersConfig) -> Result<Self, ()> {
        let mut tcp_trackers: HashMap<String, SocketAddrV4> = Default::default();

        for tracker_config in &trackers_config.0 {
            if let TrackerCommunication::TCP(address) = tracker_config.tracker_communication {
                tcp_trackers.insert(tracker_config.tracker_name.clone(), address);
            }
        }

        if tcp_trackers.is_empty() {
            return Err(());
        }

        Ok(IOBroadcastReceiverTCP {
            state: AtomicU8::new(IOBroadcastReceiverState::Stopped.into()),
            is_stop_requested: AtomicBool::new(false),
            subscriber_count: AtomicUsize::new(0),
            receiving_counter: AtomicUsize::new(0),
            start_timestamp: AtomicI64::new(0),
            broadcaster_addresses: tcp_trackers,
            pub_thread: None,
            pubsub_channel: Mutex::new(None),
        })
    }
}

impl<'a> IOBroadcastReceiver<'a> for IOBroadcastReceiverSerial<'a> {
    fn new(trackers_config: &TrackersConfig) -> Result<Self, ()> {
        let mut serial_trackers: HashMap<String, SPSettings> = Default::default();

        for tracker_config in &trackers_config.0 {
            if let TrackerCommunication::SerialPort(sp_settings) = tracker_config.tracker_communication.clone() {
                serial_trackers.insert(tracker_config.tracker_name.clone(), sp_settings);
            }
        }

        if serial_trackers.is_empty() {
            return Err(());
        }

        Ok(IOBroadcastReceiverSerial {
            state: AtomicU8::new(IOBroadcastReceiverState::Stopped.into()),
            is_stop_requested: AtomicBool::new(false),
            subscriber_count: AtomicUsize::new(0),
            receiving_counter: AtomicUsize::new(0),
            start_timestamp: AtomicI64::new(0),
            broadcaster_addresses: serial_trackers,
            pub_thread: None,
            pubsub_channel: Mutex::new(None),
        })
    }
}

#[inline]
fn increment_atomic_usize(atomic_usize: &AtomicUsize) -> usize {
    atomic_usize.fetch_add(1, Ordering::SeqCst)
}

#[inline]
fn decrement_atomic_usize(atomic_usize: &AtomicUsize) -> usize {
    atomic_usize.fetch_sub(1, Ordering::SeqCst)
}

#[inline]
fn get_atomic_usize_value(atomic_usize: &AtomicUsize) -> usize {
    atomic_usize.load(Ordering::SeqCst)
}
