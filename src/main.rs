use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::*;
use pnet::packet::ethernet::*;
use pnet::packet::*;
use pnet::packet::icmp::*;
use pnet::packet::icmp::echo_reply::*;

#[allow(unused_variables, unused_imports, unused_mut)]
use std::net::{TcpListener, TcpStream};
use std::{*, io::Read};

#[allow(clippy::unused_io_amount)]
fn main() -> std::io::Result<()> {
    let interface_name = env::args().nth(1).unwrap();
    let interface_names_match = |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(interface_names_match)
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
    let listener = TcpListener::bind("127.0.0.2:7").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut obj) => {
                let mut buf = [0;1024];
                obj.read(&mut buf)?;
                let mut pack = EchoReplyPacket::new(&mut buf).unwrap();
                let icmp_packet = IcmpPacket::new(&mut pack.payload()).unwrap();
                println!("Packet looks like: {:?}", icmp_packet);
                println!("Packet looks like: {:?}", icmp_packet.get_icmp_type());

                println!("{:?}", str::from_utf8(&buf))
            }
            Err(e) => {
            }
        }
    }
    Ok(())
}
