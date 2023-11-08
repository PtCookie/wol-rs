use std::net::{SocketAddr, UdpSocket};

use clap::Parser;

const PACKET_HEADER: [u8; 6] = [0xFF; 6];
const MULTIPLIER: usize = 16;

#[derive(Parser, Debug)]
struct Args {
    mac: String,
}

fn main() {
    let args = Args::parse();
    let mac = &args.mac;

    let mac_byte: Vec<u8> = mac
        .split(':')
        .flat_map(|x| hex::decode(x).unwrap())
        .collect();

    let mut magic_packet: Vec<u8> = PACKET_HEADER.to_vec();
    magic_packet.extend(mac_byte.repeat(MULTIPLIER));

    let host_address = SocketAddr::from(([0, 0, 0, 0], 0));
    let target_address = SocketAddr::from(([255, 255, 255, 255], 9));

    let udp_socket = UdpSocket::bind(host_address).unwrap();
    udp_socket.set_broadcast(true).unwrap();
    udp_socket.send_to(&magic_packet, target_address).unwrap();
}
