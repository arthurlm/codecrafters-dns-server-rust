use std::{
    io,
    net::{ToSocketAddrs, UdpSocket},
};

use rand::prelude::*;

use crate::{message::*, DnsError};

// const MAX_DATAGRAM_SIZE: usize = 65_507;
const MAX_DATAGRAM_SIZE: usize = 512;

#[derive(Debug)]
pub struct DnsClient {
    socket: UdpSocket,
    rng: ThreadRng,
}

impl DnsClient {
    pub fn connect<L: ToSocketAddrs, R: ToSocketAddrs>(
        local_addr: L,
        remote_addr: R,
    ) -> io::Result<Self> {
        let socket = UdpSocket::bind(local_addr)?;
        socket.connect(remote_addr)?;

        let rng = rand::thread_rng();

        Ok(Self { socket, rng })
    }

    pub fn query(&mut self, question: &QuestionSection) -> Result<AnswerSection, DnsError> {
        let id = (self.rng.next_u32() % u16::MAX as u32) as u16;

        let msg = Message {
            header: Header {
                id,
                flags: HeaderFlags {
                    qr: QrFlag::Query,
                    opcode: OpCode::Query,
                    is_authoritative_answer: false,
                    is_truncation: false,
                    is_recursion_desired: true,
                    is_recursion_available: false,
                    response_code: ResponseCode::NoError,
                },
                question_count: 1,
                answer_count: 0,
                authority_resource_record_count: 0,
                additional_resource_record_count: 0,
            },
            questions: vec![question.clone()],
            answers: vec![],
        };

        // Send msg to dns server
        let mut buf = Vec::with_capacity(4096);
        msg.encode(&mut buf)?;
        self.socket.send(&buf)?;

        // Read response
        let mut socket_data = vec![0u8; MAX_DATAGRAM_SIZE];
        let len = self.socket.recv(&mut socket_data)?;
        if len == 0 {
            return Err(DnsError::EmptyResponse);
        }

        let (_, response) = Message::parse(&socket_data[..len])?;

        // Check response content
        if response.header.id != id {
            return Err(DnsError::InvalidResponse("Invalid response ID"));
        }
        if response.answers.len() != 1 {
            return Err(DnsError::InvalidResponse("Invalid response content"));
        }

        Ok(response.answers[0].clone())
    }
}
