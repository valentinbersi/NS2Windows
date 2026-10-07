//! Motion-only DSU protocol v1001 server. Wire reference:
//! https://v1993.github.io/cemuhook-protocol/
use crate::data::output::Output;
use crate::data::output_data::OutputData;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;
use uuid::Uuid;

const VERSION: u16 = 1001;
const VERSION_MESSAGE: u32 = 0x100000;
const PORTS_MESSAGE: u32 = 0x100001;
const DATA_MESSAGE: u32 = 0x100002;
const SUBSCRIPTION_TIMEOUT: Duration = Duration::from_secs(5);
pub const DEFAULT_ADDRESS: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 26760;

pub fn endpoint(address: &str, port: u16) -> Result<SocketAddr, String> {
    if port == 0 {
        return Err("CemuHook port must be between 1 and 65535.".into());
    }
    let ip: IpAddr = address.trim().parse().map_err(|_| {
        "CemuHook bind address must be an IPv4 or IPv6 address on this PC.".to_string()
    })?;
    Ok(SocketAddr::new(ip, port))
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MotionSample {
    accel: [f32; 3],
    gyro: [f32; 3],
}

impl MotionSample {
    pub fn from_output(output: &OutputData) -> Self {
        fn axis(data: &OutputData, minus: Output, plus: Output, range: f32) -> f32 {
            // Decoder values are raw signed sensor counts / 16.384. Use the
            // hardware sensitivities (±8 g and ±2000 deg/s), not DS4 units.
            let value = data.get(plus).unwrap_or(0.0) - data.get(minus).unwrap_or(0.0);
            let converted = value * 16.384 * range / 32767.0;
            if converted.is_finite() {
                converted
            } else {
                0.0
            }
        }
        Self {
            accel: [
                axis(output, Output::AccelLeft, Output::AccelRight, 8.0),
                axis(output, Output::AccelDown, Output::AccelUp, 8.0),
                axis(output, Output::AccelBackward, Output::AccelForward, 8.0),
            ],
            gyro: [
                axis(output, Output::GyroPitchDown, Output::GyroPitchUp, 2000.0),
                axis(output, Output::GyroYawLeft, Output::GyroYawRight, 2000.0),
                axis(output, Output::GyroRollLeft, Output::GyroRollRight, 2000.0),
            ],
        }
    }
}

struct Slot {
    token: Uuid,
    mac: [u8; 6],
    sample: MotionSample,
    timestamp: u64,
    packet_number: u32,
}

#[derive(Default)]
struct Subscription {
    all: Option<Instant>,
    slots: HashMap<u8, Instant>,
    macs: HashMap<[u8; 6], Instant>,
}

impl Subscription {
    fn accepts(&self, slot: u8, mac: [u8; 6], now: Instant) -> bool {
        [
            self.all,
            self.slots.get(&slot).copied(),
            self.macs.get(&mac).copied(),
        ]
        .into_iter()
        .flatten()
        .any(|time| now.duration_since(time) < SUBSCRIPTION_TIMEOUT)
    }

    fn expire(&mut self, now: Instant) -> bool {
        let fresh = |time: &Instant| now.duration_since(*time) < SUBSCRIPTION_TIMEOUT;
        self.all = self.all.filter(fresh);
        self.slots.retain(|_, time| fresh(time));
        self.macs.retain(|_, time| fresh(time));
        self.all.is_some() || !self.slots.is_empty() || !self.macs.is_empty()
    }
}

struct Inner {
    endpoint: SocketAddr,
    socket: Option<UdpSocket>,
    worker: Option<JoinHandle<()>>,
    slots: [Option<Slot>; 4],
    subscribers: HashMap<SocketAddr, Subscription>,
    server_id: u32,
    started: Instant,
}

pub struct CemuHookServer {
    inner: Mutex<Inner>,
}

impl CemuHookServer {
    pub fn new(endpoint: SocketAddr) -> Arc<Self> {
        Arc::new(Self {
            inner: Mutex::new(Inner {
                endpoint,
                socket: None,
                worker: None,
                slots: std::array::from_fn(|_| None),
                subscribers: HashMap::new(),
                server_id: crc32(Uuid::new_v4().as_bytes()),
                started: Instant::now(),
            }),
        })
    }

    /// Serialize settings persistence with slot reservations, including startup.
    pub fn configure(
        &self,
        endpoint: SocketAddr,
        persist: impl FnOnce() -> Result<(), String>,
    ) -> Result<(), String> {
        let mut inner = self.inner.lock().unwrap();
        if endpoint != inner.endpoint && inner.slots.iter().any(Option::is_some) {
            return Err(
                "Stop all CemuHook controllers before changing its bind address or port.".into(),
            );
        }
        persist()?;
        inner.endpoint = endpoint;
        Ok(())
    }

    pub fn reserve(
        self: &Arc<Self>,
        physical_ids: &[Uuid],
    ) -> Result<CemuHookRegistration, String> {
        let mut inner = self.inner.lock().unwrap();
        let slot = inner.slots.iter().position(Option::is_none).ok_or(
            "CemuHook supports at most four controllers. Stop one before starting another.",
        )?;
        if inner.socket.is_none() {
            let socket = UdpSocket::bind(inner.endpoint).map_err(|error| format!(
                "Could not start CemuHook at {}: {error}. Check the bind address and whether another motion server uses this port.", inner.endpoint
            ))?;
            socket
                .set_nonblocking(true)
                .map_err(|error| error.to_string())?;
            inner.socket = Some(socket);
            let weak = Arc::downgrade(self);
            // The mutex owns the nonblocking socket; the worker never owns it
            // across an await. Dropping the last registration releases the port
            // synchronously, so an immediate stop/start can bind it again.
            inner.worker = Some(tokio::spawn(async move {
                let mut timer = tokio::time::interval(Duration::from_millis(2));
                timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
                loop {
                    timer.tick().await;
                    let Some(server) = weak.upgrade() else { break };
                    server.poll();
                }
            }));
        }
        let token = Uuid::new_v4();
        inner.slots[slot] = Some(Slot {
            token,
            mac: synthetic_mac(physical_ids),
            sample: MotionSample::default(),
            timestamp: 0,
            packet_number: 0,
        });
        Ok(CemuHookRegistration {
            server: self.clone(),
            slot: slot as u8,
            token,
        })
    }

    fn poll(&self) {
        let mut inner = self.inner.lock().unwrap();
        let mut buffer = [0u8; 2048];
        // Bound work per tick so malformed traffic cannot starve emulation.
        for _ in 0..64 {
            let Some(socket) = &inner.socket else { return };
            match socket.recv_from(&mut buffer) {
                Ok((len, peer)) => inner.handle(&buffer[..len], peer),
                Err(error) if error.kind() == ErrorKind::WouldBlock => break,
                Err(error) => {
                    eprintln!("CemuHook receive failed: {error}");
                    break;
                }
            }
        }
        inner
            .subscribers
            .retain(|_, subscription| subscription.expire(Instant::now()));
    }

    fn release(&self, slot: u8, token: Uuid) {
        let mut inner = self.inner.lock().unwrap();
        if inner.slots[slot as usize]
            .as_ref()
            .is_some_and(|entry| entry.token == token)
        {
            inner.slots[slot as usize] = None;
            // A slot subscription must not accidentally follow a replacement
            // controller. MAC/all subscriptions retain their intended meaning.
            for subscription in inner.subscribers.values_mut() {
                subscription.slots.remove(&slot);
            }
        }
        if inner.slots.iter().all(Option::is_none) {
            if let Some(worker) = inner.worker.take() {
                worker.abort();
            }
            inner.socket = None;
            inner.subscribers.clear();
        }
    }
}

impl Drop for CemuHookServer {
    fn drop(&mut self) {
        if let Some(worker) = self.inner.get_mut().unwrap().worker.take() {
            worker.abort();
        }
    }
}

pub struct CemuHookRegistration {
    server: Arc<CemuHookServer>,
    slot: u8,
    token: Uuid,
}

impl CemuHookRegistration {
    pub fn publisher(&self) -> MotionPublisher {
        MotionPublisher {
            server: self.server.clone(),
            slot: self.slot,
            token: self.token,
        }
    }
}

impl Drop for CemuHookRegistration {
    fn drop(&mut self) {
        self.server.release(self.slot, self.token);
    }
}

#[derive(Clone)]
pub struct MotionPublisher {
    server: Arc<CemuHookServer>,
    slot: u8,
    token: Uuid,
}

impl MotionPublisher {
    pub fn publish(&self, output: &OutputData, fresh_sample: bool) {
        let mut inner = self.server.inner.lock().unwrap();
        let timestamp = inner.started.elapsed().as_micros().min(u64::MAX as u128) as u64;
        let server_id = inner.server_id;
        let Some(slot) = inner.slots[self.slot as usize]
            .as_mut()
            .filter(|entry| entry.token == self.token)
        else {
            return;
        };
        if fresh_sample {
            slot.sample = MotionSample::from_output(output);
            slot.timestamp = timestamp.max(slot.timestamp.saturating_add(1));
        }
        if slot.timestamp == 0 {
            return;
        }
        slot.packet_number = slot.packet_number.wrapping_add(1);
        let mac = slot.mac;
        let packet = data_packet(server_id, self.slot, slot);
        let now = Instant::now();
        if let Some(socket) = &inner.socket {
            for (peer, subscription) in &inner.subscribers {
                if subscription.accepts(self.slot, mac, now) {
                    send(socket, &packet, *peer);
                }
            }
        }
    }
}

impl Inner {
    fn handle(&mut self, packet: &[u8], peer: SocketAddr) {
        let Some((message, body)) = parse(packet) else {
            return;
        };
        match message {
            VERSION_MESSAGE if body.is_empty() => {
                self.reply(
                    peer,
                    packet_for(self.server_id, message, &VERSION.to_le_bytes()),
                );
            }
            PORTS_MESSAGE if body.len() >= 4 => {
                let count = i32::from_le_bytes(body[..4].try_into().unwrap());
                if !(1..=4).contains(&count)
                    || body.len() != 4 + count as usize
                    || body[4..].iter().any(|slot| *slot >= 4)
                {
                    return;
                }
                for &slot in &body[4..] {
                    let mut body = metadata(slot, self.slots[slot as usize].as_ref()).to_vec();
                    body.push(0);
                    self.reply(peer, packet_for(self.server_id, message, &body));
                }
            }
            DATA_MESSAGE if body.len() == 8 => {
                let flags = body[0];
                if flags & !3 != 0 || (flags & 1 != 0 && body[1] >= 4) {
                    return;
                }
                let mac: [u8; 6] = body[2..8].try_into().unwrap();
                let slot_connected = flags & 1 != 0 && self.slots[body[1] as usize].is_some();
                let mac_connected =
                    flags & 2 != 0 && self.slots.iter().flatten().any(|slot| slot.mac == mac);
                if flags != 0 && !slot_connected && !mac_connected {
                    return;
                }
                let now = Instant::now();
                // Bound subscription storage; clients renew at least every 5s.
                if !self.subscribers.contains_key(&peer) && self.subscribers.len() >= 256 {
                    return;
                }
                let subscription = self.subscribers.entry(peer).or_default();
                if flags == 0 {
                    subscription.all = Some(now);
                }
                if slot_connected {
                    subscription.slots.insert(body[1], now);
                }
                if mac_connected {
                    subscription.macs.insert(mac, now);
                }
            }
            _ => {}
        }
    }

    fn reply(&self, peer: SocketAddr, packet: Vec<u8>) {
        if let Some(socket) = &self.socket {
            send(socket, &packet, peer);
        }
    }
}

fn send(socket: &UdpSocket, packet: &[u8], peer: SocketAddr) {
    if let Err(error) = socket.send_to(packet, peer) {
        if error.kind() != ErrorKind::WouldBlock {
            eprintln!("CemuHook send failed: {error}");
        }
    }
}

fn synthetic_mac(ids: &[Uuid]) -> [u8; 6] {
    // Locally administered unicast identity, deterministic for a bound setup.
    let mut hash = 0xcbf29ce484222325u64;
    for byte in ids.iter().flat_map(|id| id.as_bytes()) {
        hash = (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3);
    }
    let mut mac: [u8; 6] = hash.to_le_bytes()[..6].try_into().unwrap();
    mac[0] = (mac[0] & 0xfc) | 2;
    mac
}

fn metadata(slot_id: u8, slot: Option<&Slot>) -> [u8; 11] {
    let mut body = [0u8; 11];
    body[0] = slot_id;
    if let Some(slot) = slot {
        body[1] = 2; // Connected.
        body[2] = 2; // Full gyro.
        body[3] = 2; // Bluetooth.
        body[4..10].copy_from_slice(&slot.mac);
        // Battery unknown; do not fabricate a hardware battery level.
    }
    body
}

fn data_packet(server_id: u32, slot_id: u8, slot: &Slot) -> Vec<u8> {
    let mut body = [0u8; 80];
    body[..11].copy_from_slice(&metadata(slot_id, Some(slot)));
    body[11] = 1;
    body[12..16].copy_from_slice(&slot.packet_number.to_le_bytes());
    body[20..24].fill(128);
    body[48..56].copy_from_slice(&slot.timestamp.to_le_bytes());
    for (index, value) in slot
        .sample
        .accel
        .into_iter()
        .chain(slot.sample.gyro)
        .enumerate()
    {
        body[56 + index * 4..60 + index * 4].copy_from_slice(&value.to_le_bytes());
    }
    packet_for(server_id, DATA_MESSAGE, &body)
}

fn packet_for(server_id: u32, message: u32, body: &[u8]) -> Vec<u8> {
    let mut packet = Vec::with_capacity(20 + body.len());
    packet.extend_from_slice(b"DSUS");
    packet.extend_from_slice(&VERSION.to_le_bytes());
    packet.extend_from_slice(&((body.len() + 4) as u16).to_le_bytes());
    packet.extend_from_slice(&[0; 4]);
    packet.extend_from_slice(&server_id.to_le_bytes());
    packet.extend_from_slice(&message.to_le_bytes());
    packet.extend_from_slice(body);
    let crc = crc32(&packet);
    packet[8..12].copy_from_slice(&crc.to_le_bytes());
    packet
}

fn parse(packet: &[u8]) -> Option<(u32, &[u8])> {
    if packet.len() < 20
        || &packet[..4] != b"DSUC"
        || u16::from_le_bytes(packet[4..6].try_into().ok()?) != VERSION
    {
        return None;
    }
    let length = 16 + u16::from_le_bytes(packet[6..8].try_into().ok()?) as usize;
    if length < 20 || length > packet.len() {
        return None;
    }
    let packet = &packet[..length];
    let expected = u32::from_le_bytes(packet[8..12].try_into().ok()?);
    let actual = crc32_bytes(
        packet
            .iter()
            .enumerate()
            .map(|(i, byte)| if (8..12).contains(&i) { 0 } else { *byte }),
    );
    if actual != expected {
        return None;
    }
    Some((
        u32::from_le_bytes(packet[16..20].try_into().ok()?),
        &packet[20..],
    ))
}

fn crc32(bytes: &[u8]) -> u32 {
    crc32_bytes(bytes.iter().copied())
}

fn crc32_bytes(bytes: impl Iterator<Item = u8>) -> u32 {
    let mut crc = !0u32;
    for byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xedb88320 & (0u32.wrapping_sub(crc & 1)));
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    fn client_packet(message: u32, body: &[u8]) -> Vec<u8> {
        let mut packet = packet_for(42, message, body);
        packet[..4].copy_from_slice(b"DSUC");
        packet[8..12].fill(0);
        let crc = crc32(&packet);
        packet[8..12].copy_from_slice(&crc.to_le_bytes());
        packet
    }

    fn test_server() -> Arc<CemuHookServer> {
        CemuHookServer::new("127.0.0.1:0".parse().unwrap())
    }

    fn address(server: &CemuHookServer) -> SocketAddr {
        server
            .inner
            .lock()
            .unwrap()
            .socket
            .as_ref()
            .unwrap()
            .local_addr()
            .unwrap()
    }

    #[test]
    fn endpoint_validation() {
        assert_eq!(
            endpoint(" 127.0.0.1 ", 26760).unwrap().to_string(),
            "127.0.0.1:26760"
        );
        assert_eq!(endpoint("::1", 65535).unwrap().to_string(), "[::1]:65535");
        assert!(endpoint("localhost", 26760).is_err());
        assert!(endpoint("192.168.1.256", 26760).is_err());
        assert!(endpoint("127.0.0.1", 0).is_err());
    }

    #[test]
    fn crc_and_motion_packet_wire_layout() {
        assert_eq!(crc32(b"123456789"), 0xcbf43926);
        let slot = Slot {
            token: Uuid::nil(),
            mac: [2, 1, 2, 3, 4, 5],
            sample: MotionSample {
                accel: [1.0, -2.0, 3.0],
                gyro: [-4.0, 5.0, -6.0],
            },
            timestamp: 123456,
            packet_number: 9,
        };
        let packet = data_packet(123, 2, &slot);
        assert_eq!(packet.len(), 100);
        assert_eq!(&packet[..8], &[b'D', b'S', b'U', b'S', 0xe9, 3, 84, 0]);
        assert_eq!(&packet[12..20], &[123, 0, 0, 0, 2, 0, 16, 0]);
        assert_eq!(&packet[20..24], &[2, 2, 2, 2]);
        assert_eq!(&packet[24..30], &slot.mac);
        assert_eq!(&packet[31..36], &[1, 9, 0, 0, 0]);
        assert!(packet[36..40].iter().all(|byte| *byte == 0));
        assert_eq!(&packet[40..44], &[128; 4]);
        assert!(packet[44..68].iter().all(|byte| *byte == 0));
        assert_eq!(
            u64::from_le_bytes(packet[68..76].try_into().unwrap()),
            123456
        );
        for (i, expected) in [1.0, -2.0, 3.0, -4.0, 5.0, -6.0].into_iter().enumerate() {
            assert_eq!(
                f32::from_le_bytes(packet[76 + i * 4..80 + i * 4].try_into().unwrap()),
                expected
            );
        }
        let checksum = u32::from_le_bytes(packet[8..12].try_into().unwrap());
        let mut zeroed = packet;
        zeroed[8..12].fill(0);
        assert_eq!(checksum, crc32(&zeroed));
    }

    #[test]
    fn malformed_headers_and_lengths_are_ignored() {
        let valid = client_packet(DATA_MESSAGE, &[0; 8]);
        for length in 0..valid.len() {
            assert!(parse(&valid[..length]).is_none());
        }
        for offset in [0, 4, 6, 8, 16, 20] {
            let mut invalid = valid.clone();
            invalid[offset] ^= 0xff;
            assert!(parse(&invalid).is_none());
        }
        let mut trailing = valid.clone();
        trailing.extend_from_slice(&[1, 2, 3]);
        assert_eq!(parse(&trailing), Some((DATA_MESSAGE, &[0u8; 8][..])));
        assert!(parse(&packet_for(0, DATA_MESSAGE, &[0; 8])).is_none());
    }

    #[test]
    fn mapped_axes_use_hardware_units_and_preserve_signs() {
        let counts = 32767.0 / 16.384;
        let sample = MotionSample::from_output(&OutputData::new(hashmap! {
            Output::AccelRight => counts, Output::AccelDown => counts / 2.0,
            Output::AccelForward => counts / 4.0, Output::GyroPitchDown => counts,
            Output::GyroYawRight => counts / 2.0, Output::GyroRollLeft => counts / 4.0,
        }));
        assert_eq!(sample.accel, [8.0, -4.0, 2.0]);
        assert_eq!(sample.gyro, [-2000.0, 1000.0, -500.0]);
        let cancelled = MotionSample::from_output(&OutputData::new(hashmap! {
            Output::AccelRight => 10.0, Output::AccelLeft => 10.0,
            Output::GyroPitchUp => f32::NAN,
        }));
        assert_eq!(cancelled, MotionSample::default());
    }

    #[tokio::test]
    async fn slots_are_reused_and_last_release_closes_port_immediately() {
        let server = test_server();
        let ids = [Uuid::new_v4(), Uuid::new_v4()];
        let mut registrations = (0..4)
            .map(|_| Some(server.reserve(&ids).unwrap()))
            .collect::<Vec<_>>();
        for (i, registration) in registrations.iter().enumerate() {
            assert_eq!(registration.as_ref().unwrap().slot, i as u8);
        }
        let bound = address(&server);
        assert!(
            server
                .reserve(&ids)
                .err()
                .unwrap()
                .contains("four controllers")
        );
        let old_publisher = registrations[1].as_ref().unwrap().publisher();
        let old_mac = server.inner.lock().unwrap().slots[1].as_ref().unwrap().mac;
        registrations[1] = None;
        let replacement = server.reserve(&ids).unwrap();
        assert_eq!(replacement.slot, 1);
        assert_eq!(
            server.inner.lock().unwrap().slots[1].as_ref().unwrap().mac,
            old_mac
        );
        old_publisher.publish(&OutputData::default(), true);
        assert_eq!(
            server.inner.lock().unwrap().slots[1]
                .as_ref()
                .unwrap()
                .timestamp,
            0
        );
        drop(registrations);
        assert!(UdpSocket::bind(bound).is_err());
        drop(replacement);
        assert!(UdpSocket::bind(bound).is_ok());
        assert!(server.inner.lock().unwrap().socket.is_none());
    }

    #[tokio::test]
    async fn occupied_port_and_settings_changes_do_not_leak_or_mutate_state() {
        let occupied = UdpSocket::bind("127.0.0.1:0").unwrap();
        let server = CemuHookServer::new(occupied.local_addr().unwrap());
        assert!(
            server
                .reserve(&[Uuid::nil()])
                .err()
                .unwrap()
                .contains("Could not start CemuHook")
        );
        assert!(
            server
                .inner
                .lock()
                .unwrap()
                .slots
                .iter()
                .all(Option::is_none)
        );
        drop(occupied);
        let registration = server.reserve(&[Uuid::nil()]).unwrap();
        let current = server.inner.lock().unwrap().endpoint;
        let other = endpoint("::1", 26761).unwrap();
        assert!(
            server
                .configure(other, || panic!(
                    "must not persist an active endpoint change"
                ))
                .is_err()
        );
        assert_eq!(server.inner.lock().unwrap().endpoint, current);
        assert!(server.configure(current, || Ok(())).is_ok());
        drop(registration);
        assert!(
            server
                .configure(other, || Err("save failed".into()))
                .is_err()
        );
        assert_eq!(server.inner.lock().unwrap().endpoint, current);
        server.configure(other, || Ok(())).unwrap();
        assert_eq!(server.inner.lock().unwrap().endpoint, other);
    }

    #[tokio::test]
    async fn subscriptions_match_slot_mac_and_all_and_expire() {
        let server = test_server();
        let _registration = server.reserve(&[Uuid::nil()]).unwrap();
        let peer: SocketAddr = "127.0.0.1:12345".parse().unwrap();
        let mut inner = server.inner.lock().unwrap();
        let mac = inner.slots[0].as_ref().unwrap().mac;
        inner.handle(
            &client_packet(DATA_MESSAGE, &[1, 0, 0, 0, 0, 0, 0, 0]),
            peer,
        );
        let now = Instant::now();
        assert!(inner.subscribers[&peer].accepts(0, mac, now));
        assert!(!inner.subscribers[&peer].accepts(1, mac, now));
        inner.subscribers.clear();
        let mut by_mac = [2, 0, 0, 0, 0, 0, 0, 0];
        by_mac[2..].copy_from_slice(&mac);
        inner.handle(&client_packet(DATA_MESSAGE, &by_mac), peer);
        assert!(inner.subscribers[&peer].accepts(2, mac, Instant::now()));
        assert!(!inner.subscribers[&peer].accepts(0, [0; 6], Instant::now()));
        inner.handle(&client_packet(DATA_MESSAGE, &[0; 8]), peer);
        assert!(inner.subscribers[&peer].accepts(3, [0; 6], Instant::now()));
        assert!(
            !inner
                .subscribers
                .get_mut(&peer)
                .unwrap()
                .expire(Instant::now() + SUBSCRIPTION_TIMEOUT)
        );
        inner.subscribers.clear();
        for body in [
            &[1, 4, 0, 0, 0, 0, 0, 0][..],
            &[8; 8][..],
            &[0; 7][..],
            &[1, 3, 0, 0, 0, 0, 0, 0][..],
        ] {
            inner.handle(&client_packet(DATA_MESSAGE, body), peer);
            assert!(inner.subscribers.is_empty());
        }
    }

    async fn receive(client: &tokio::net::UdpSocket) -> Vec<u8> {
        let mut packet = [0; 2048];
        let length = tokio::time::timeout(Duration::from_secs(2), client.recv(&mut packet))
            .await
            .unwrap()
            .unwrap();
        packet[..length].to_vec()
    }

    #[tokio::test]
    async fn udp_client_discovers_and_receives_motion_with_sample_timestamps() {
        let server = test_server();
        let registration = server.reserve(&[Uuid::nil()]).unwrap();
        let publisher = registration.publisher();
        let client = tokio::net::UdpSocket::bind("127.0.0.1:0").await.unwrap();
        client.connect(address(&server)).await.unwrap();
        client
            .send(&client_packet(VERSION_MESSAGE, &[]))
            .await
            .unwrap();
        let version = receive(&client).await;
        assert_eq!(&version[20..], &VERSION.to_le_bytes());
        client
            .send(&client_packet(PORTS_MESSAGE, &[2, 0, 0, 0, 0, 3]))
            .await
            .unwrap();
        let connected = receive(&client).await;
        let disconnected = receive(&client).await;
        assert_eq!(&connected[20..24], &[0, 2, 2, 2]);
        assert_eq!(&disconnected[20..24], &[3, 0, 0, 0]);
        client
            .send(&client_packet(DATA_MESSAGE, &[1, 0, 0, 0, 0, 0, 0, 0]))
            .await
            .unwrap();
        // A subsequent version response establishes that registration was read.
        client
            .send(&client_packet(VERSION_MESSAGE, &[]))
            .await
            .unwrap();
        receive(&client).await;
        let output = OutputData::new(hashmap! { Output::AccelRight => 4096.0 / 16.384 });
        publisher.publish(&output, true);
        let first = receive(&client).await;
        assert_eq!(first.len(), 100);
        assert!((f32::from_le_bytes(first[76..80].try_into().unwrap()) - 1.0).abs() < 0.001);
        publisher.publish(&OutputData::default(), false);
        let repeat = receive(&client).await;
        assert_eq!(&first[68..100], &repeat[68..100]);
        assert_ne!(&first[32..36], &repeat[32..36]);
        publisher.publish(&output, true);
        let fresh = receive(&client).await;
        assert!(
            u64::from_le_bytes(fresh[68..76].try_into().unwrap())
                > u64::from_le_bytes(first[68..76].try_into().unwrap())
        );
    }
}
