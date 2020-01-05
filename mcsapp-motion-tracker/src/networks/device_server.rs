#[derive(Debug)]
pub struct UDPv4Multicast<'a> {
    pub name: String,
    state: AtomicU8,
    is_stop_requested: AtomicBool,
    subscriber_count: AtomicUsize,
    receiving_counter: AtomicUsize,
    sending_counter: AtomicUsize,
    sending_queue: SegQueue<Bytes>,
    start_timestamp: AtomicI64,
    bind_address: SocketAddrV4,
    multicast_address: Mutex<Option<SocketAddrV4>>,
    pub_thread: Option<ScopedJoinHandle<'a, ()>>,
    sending_thread: Option<ScopedJoinHandle<'a, ()>>,
    pubsub_channel: Mutex<Option<(Sender<Bytes>, MessageReceivingChannel)>>,
}
