use std::{
    io,
    net::{SocketAddr, UdpSocket},
};

use dns_starter_rust::message::*;

fn main() -> io::Result<()> {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053")?;
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let response = handle_query(&buf[..size]);
                send_response(&udp_socket, &source, &response)?;
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn handle_query(input: &[u8]) -> Message {
    let (_, query) = Message::parse(input).expect("Invalid DNS query");
    println!("query: {query:#?}");

    Message {
        header: Header {
            id: query.header.id,
            flags: HeaderFlags {
                qr: QrFlag::Reply,
                opcode: query.header.flags.opcode,
                is_authoritative_answer: false,
                is_truncation: false,
                is_recursion_desired: query.header.flags.is_recursion_desired,
                is_recursion_available: false,
                response_code: if query.header.flags.opcode == OpCode::Query {
                    ResponseCode::NoError
                } else {
                    ResponseCode::NotImplemented
                },
            },
            question_count: query.questions.len() as u16,
            answer_count: query.questions.len() as u16,
            authority_resource_record_count: 0,
            additional_resource_record_count: 0,
        },
        questions: query
            .questions
            .iter()
            .map(|question| QuestionSection {
                labels: question.labels.clone(),
                label_offset: None,
                rr_type: ResourceRecordType::A,
                rr_class: ResourceRecordClass::IN,
            })
            .collect(),
        answers: query
            .questions
            .iter()
            .map(|question| AnswerSection {
                labels: question.labels.clone(),
                rr_type: ResourceRecordType::A,
                rr_class: ResourceRecordClass::IN,
                ttl: 60,
                data: vec![8, 8, 8, 8],
            })
            .collect(),
    }
}

fn send_response(
    udp_socket: &UdpSocket,
    source: &SocketAddr,
    response: &Message,
) -> io::Result<()> {
    let mut buffer = Vec::with_capacity(4096);

    response.encode(&mut buffer).expect("Fail to write buffer");
    udp_socket.send_to(&buffer, source)?;

    Ok(())
}
