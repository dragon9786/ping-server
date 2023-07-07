use pnet::packet::icmp::echo_reply::MutableEchoReplyPacket;
use pnet::packet::icmp::IcmpCode;
use pnet::packet::icmp::IcmpTypes;
use pnet::packet::ip::IpNextHeaderProtocol;
use pnet::transport::icmp_packet_iter;
use pnet::transport::transport_channel;
use pnet::transport::*;

use std::*;

fn main() -> std::io::Result<()> {
    let (mut tx, mut rx) = match transport_channel(
        64,
        pnet::transport::TransportChannelType::Layer4(TransportProtocol::Ipv4(
            IpNextHeaderProtocol::new(1),
        )),
    ) {
        Ok((tx, rx)) => (tx, rx),
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    };
    let mut icmp_iter = icmp_packet_iter(&mut rx);
    loop {
        match icmp_iter.next() {
            Ok(ping_request) => {
                let destination = ping_request.1;
                let mut payload = [0u8; 64];
                println!("Got IcmpEchoRequest from source: {:?}", destination);

                let mut ping_response: MutableEchoReplyPacket =
                    MutableEchoReplyPacket::new(&mut payload).unwrap();
                ping_response.set_icmp_type(IcmpTypes::EchoReply);
                ping_response.set_payload(b"hello");
                ping_response.set_icmp_code(IcmpCode(0));
                tx.send_to(ping_response, destination).unwrap();
                println!("Sending IcmpEchoReply to destination: {:?}", destination,);
            }
            Err(e) => {
                eprint!("Error: {:?}", e);
            }
        }
    }
}
