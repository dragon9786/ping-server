#![allow(unused_imports)]

use pnet::datalink::Channel::Ethernet;
use pnet::datalink::*;
use pnet::packet::ethernet::*;
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::*;
use pnet::transport::transport_channel;
use pnet::transport::TransportReceiver;
use pnet::transport::TransportSender;
use pnet::{datalink, packet::icmp::MutableIcmpPacket};

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

    // println!("Interface is {:?}", interface.to_string());
    // let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
    //     Ok(Ethernet(tx, rx)) => (tx, rx),
    //     Ok(_) => panic!("Unhandled channel type"),
    //     Err(e) => panic!(
    //         "An error occurred when creating the datalink channel: {:?}",
    //         e
    //     ),
    // };

    // loop {
    //     match rx.next() {
    //         Ok(packet) => {
    //             let packet: IcmpPacket = IcmpPacket::new(packet).unwrap();
    //             //println!("Packet: Source {:?}", packet.payload());
    //             tx.build_and_send(1, packet.packet().len(), &mut |mut new_packet| {
    //                 let mut new_packet: MutableIcmpPacket =
    //                     MutableIcmpPacket::new(new_packet).unwrap();

    //                 // Create a clone of the original packet
    //                 new_packet.clone_from(&packet);

    //                 new_packet.set_icmp_code(icmp::IcmpCode::new(0));
    //                 new_packet.set_icmp_type(icmp::IcmpTypes::EchoReply);
    //             });
    //         }
    //         Err(e) => {
    //             eprint!("Error: {:?}", e);
    //         }
    //     }
    // }

    let (tx, rx) = match transport_channel(64, pnet::transport::TransportChannelType::Layer4(())) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => {
            panic!("Error");
        }
    };
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet: IcmpPacket = IcmpPacket::new(packet).unwrap();
                //println!("Packet: Source {:?}", packet.payload());
                tx.build_and_send(1, packet.packet().len(), &mut |mut new_packet| {
                    let mut new_packet: MutableIcmpPacket =
                        MutableIcmpPacket::new(new_packet).unwrap();

                    // Create a clone of the original packet
                    new_packet.clone_from(&packet);

                    new_packet.set_icmp_code(icmp::IcmpCode::new(0));
                    new_packet.set_icmp_type(icmp::IcmpTypes::EchoReply);
                });
            }
            Err(e) => {
                eprint!("Error: {:?}", e);
            }
        }
    }

    Ok(())
}
