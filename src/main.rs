use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::*;
use pnet::packet::ethernet::*;
use pnet::packet::*;

#[allow(unused_variables, unused_imports, unused_mut)]
use std::net::{TcpListener, TcpStream};
use std::*;

fn main() -> std::io::Result<()> {
    let interface_name = env::args().nth(1).unwrap();
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();

    println!("Interface is {:?}", interface.to_string());
    let (tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {:?}",
            e
        ),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                println!("Packet: Source {:?}", packet.payload());
            }
            Err(e) => {
                eprint!("Error: {:?}", e);
            }
        }
    }
}
