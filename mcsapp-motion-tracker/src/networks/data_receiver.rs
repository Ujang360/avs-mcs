use crossbeam_channel::{Receiver, Sender};
use crossbeam_utils::thread::ScopedJoinHandle;
use mcslib_common::bytes::Bytes;
use mcslib_common::types::{SerialPortSettings as SPSettings, TrackersConfig};
use std::collections::HashMap;
use std::mem;
use std::net::SocketAddrV4;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU8, AtomicUsize, Ordering};
use std::sync::Mutex;

pub type DataReceiverChannel = Receiver<Bytes>;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum DataReceiverState {
    Stopped = 0,
    Starting = 1,
    Started = 2,
    Stopping = 3,
}

#[derive(Debug)]
pub struct DataReceiverUDP<'a> {
    state: AtomicU8,
    is_stop_requested: AtomicBool,
    subscriber_count: AtomicUsize,
    receiving_counter: AtomicUsize,
    start_timestamp: AtomicI64,
    broadcaster_addresses: HashMap<String, TrackersConfig>,
    pub_thread: Option<Vec<ScopedJoinHandle<'a, ()>>>,
    pubsub_channel: Mutex<Option<(Sender<Bytes>, DataReceiverChannel)>>,
}

#[derive(Debug)]
pub struct DataReceiverTCP<'a> {
    state: AtomicU8,
    is_stop_requested: AtomicBool,
    subscriber_count: AtomicUsize,
    receiving_counter: AtomicUsize,
    start_timestamp: AtomicI64,
    broadcaster_addresses: HashMap<String, SocketAddrV4>,
    pub_thread: Option<Vec<ScopedJoinHandle<'a, ()>>>,
    pubsub_channel: Mutex<Option<(Sender<Bytes>, DataReceiverChannel)>>,
}

#[derive(Debug)]
pub struct DataReceiverSerial<'a> {
    state: AtomicU8,
    is_stop_requested: AtomicBool,
    subscriber_count: AtomicUsize,
    receiving_counter: AtomicUsize,
    start_timestamp: AtomicI64,
    broadcaster_addresses: HashMap<String, SPSettings>,
    pub_thread: Option<Vec<ScopedJoinHandle<'a, ()>>>,
    pubsub_channel: Mutex<Option<(Sender<Bytes>, DataReceiverChannel)>>,
}

impl Into<u8> for DataReceiverState {
    #[inline]
    fn into(self) -> u8 {
        self as u8
    }
}

impl From<u8> for DataReceiverState {
    #[inline]
    fn from(value: u8) -> DataReceiverState {
        match value {
            0 => DataReceiverState::Stopped,
            1 => DataReceiverState::Starting,
            2 => DataReceiverState::Started,
            3 => DataReceiverState::Stopping,
            _ => panic!("Invalid state value!"),
        }
    }
}

impl PartialEq for DataReceiverState {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl Eq for DataReceiverState {}

#[allow(dead_code)]
#[inline]
fn increment_atomic_usize(atomic_usize: &AtomicUsize) -> usize {
    atomic_usize.fetch_add(1, Ordering::SeqCst)
}

#[allow(dead_code)]
#[inline]
fn decrement_atomic_usize(atomic_usize: &AtomicUsize) -> usize {
    atomic_usize.fetch_sub(1, Ordering::SeqCst)
}

#[allow(dead_code)]
#[inline]
fn get_atomic_usize_value(atomic_usize: &AtomicUsize) -> usize {
    atomic_usize.load(Ordering::SeqCst)
}
