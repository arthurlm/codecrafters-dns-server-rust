use std::{io, mem, net::UdpSocket};

use dns_starter_rust::message::header::*;

fn main() -> io::Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053")?;
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let (_, query_header) = Header::parse(&buf[..size]).expect("Invalid query header");
                println!("query_header: {query_header:#?}");

                let mut response_buffer = Vec::with_capacity(mem::size_of::<Header>());
                let response_header = Header {
                    id: query_header.id,
                    flags: HeaderFlags {
                        qr: QrFlag::Reply,
                        opcode: OpCode::Query,
                        is_authoritative_answer: false,
                        is_truncation: false,
                        is_recursion_desired: false,
                        is_recursion_available: false,
                        response_code: ResponseCode::default(),
                    },
                    question_count: 0,
                    answer_count: 0,
                    authority_resource_record_count: 0,
                    additional_resource_record_count: 0,
                };
                response_header
                    .encode(&mut response_buffer)
                    .expect("Fail to write buffer");

                println!("buf: {:?}", &response_buffer);
                udp_socket.send_to(&response_buffer, source)?;
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }

    Ok(())
}
