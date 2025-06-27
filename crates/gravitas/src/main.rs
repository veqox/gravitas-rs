use std::net::UdpSocket;

use dns::{
    class::Class,
    domain::{Domain, Label},
    header::MessageType,
    packet::Packet,
    record::{Record, RecordData},
    span::Span,
    r#type::Type,
};
use log::{debug, error, info};

const LISTEN_ADDR: &str = "0.0.0.0:5300";

fn main() {
    env_logger::init();

    let socket = UdpSocket::bind(LISTEN_ADDR).expect("couldn't bind to address");
    info!("listening on {}", LISTEN_ADDR);

    let mut buf = [0; 4096];

    loop {
        let (len, addr) = match socket.recv_from(&mut buf) {
            Ok(x) => x,
            Err(e) => {
                error!("failed to receive data: {}", e);
                continue;
            }
        };

        debug!("received {} bytes from {}", len, addr.ip());

        debug!(
            "{:#?}",
            &buf[..len]
                .iter()
                .enumerate()
                .map(|(i, x)| (i, *x as char))
                .collect::<Vec<_>>()
        );

        let packet = match Packet::from_buf(&buf[..len]) {
            Err(err) => {
                error!("failed to parse packet {:?}", err);
                continue;
            }
            Ok(packet) => packet,
        };

        debug!("{:#?}", packet);

        let response = {
            packet.header.flags.message_type = MessageType::Response;
            packet.add_answer(Record::Record {
                name: Domain {
                    labels: vec![Label {
                        data: Span::Owned {
                            data: "kurwa".as_bytes().to_vec().into_boxed_slice(),
                        },
                    }],
                },
                r#type: Type::A,
                class: Class::IN,
                ttl: 3600,
                data: RecordData::A {
                    address: Span::Owned {
                        data: vec![1, 1, 1, 1].into_boxed_slice(),
                    },
                },
            });

            packet
        };

        // let response = match packet.header.flags.recursion_desired {
        //     true => {
        //         let len = packet.write_to(&mut buf).unwrap();

        //         _ = socket.send_to(&buf[..len], "");

        //         let len = socket.recv(&mut buf).unwrap();

        //         Packet::from_buf(&buf[..len]).unwrap()
        //     }
        //     false => {
        //         todo!();
        //     }
        // };

        let len = response.write_to(&mut buf).unwrap();

        _ = socket.send_to(&buf[..len], addr);
    }
}
