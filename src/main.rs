mod core;
mod logging;

use std::{collections::HashMap, io::Cursor, net::UdpSocket};

use deku::prelude::*;
use tracing::debug;

use crate::core::*;

fn main() {
    crate::logging::setup_console_log();

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                debug!("Received {} bytes from {}", size, source);

                let (_res, mut hdr) = DnsHeader::from_bytes((&buf, 0)).unwrap();

                debug!("Received DNS Header: {:?}", hdr);

                hdr.qr = true;

                debug!("Sending response...");
                let mut packet = DnsPacket {
                    header: hdr,
                    questions: vec![DnsQuestion {
                        name: "www.google.com.".parse().unwrap(),
                        r#type: DnsType::A,
                        class: DnsClass::In,
                    }],
                    answers: vec![DnsRecord {
                        name: "www.google.com.".parse().unwrap(),
                        r#type: DnsType::A,
                        class: DnsClass::In,
                        ttl: 3600,
                        len: 4,
                        data: vec![DnsRData::IP("127.0.0.1".parse().unwrap())],
                    }],
                    ..Default::default()
                };
                packet.header.qdcount = 1;
                packet.header.ancount = 1;

                debug!("Response: {:?}", packet);

                let mut buf = Vec::new();
                let cursor = Cursor::new(&mut buf);
                let mut writer = Writer::new(cursor);

                packet.to_writer(&mut writer, &mut HashMap::new()).unwrap();
                udp_socket
                    .send_to(&buf, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                debug!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
