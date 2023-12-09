use std::io::{self, Write};

use nom::{
    bits::{bits, streaming::take},
    error::Error,
    number::complete::be_u16,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Header {
    pub id: u16,
    pub flags: HeaderFlags,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_resource_record_count: u16,
    pub additional_resource_record_count: u16,
}

impl Header {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, id) = be_u16(input)?;
        let (input, flags) = HeaderFlags::parse(input)?;
        let (input, question_count) = be_u16(input)?;
        let (input, answer_count) = be_u16(input)?;
        let (input, authority_resource_record_count) = be_u16(input)?;
        let (input, additional_resource_record_count) = be_u16(input)?;

        Ok((
            input,
            Self {
                id,
                flags,
                question_count,
                answer_count,
                authority_resource_record_count,
                additional_resource_record_count,
            },
        ))
    }

    pub fn encode<W: Write>(&self, buf: &mut W) -> io::Result<()> {
        let mut flags = 0;
        flags |= (self.flags.qr as u16) << 15;
        flags |= (self.flags.opcode as u16) << 11;
        flags |= (self.flags.is_authoritative_answer as u16) << 10;
        flags |= (self.flags.is_truncation as u16) << 9;
        flags |= (self.flags.is_recursion_desired as u16) << 8;
        flags |= (self.flags.is_recursion_available as u16) << 7;
        flags |= self.flags.response_code as u16;

        buf.write(&self.id.to_be_bytes())?;
        buf.write(&flags.to_be_bytes())?;
        buf.write(&self.question_count.to_be_bytes())?;
        buf.write(&self.answer_count.to_be_bytes())?;
        buf.write(&self.authority_resource_record_count.to_be_bytes())?;
        buf.write(&self.additional_resource_record_count.to_be_bytes())?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HeaderFlags {
    pub qr: QrFlag,
    pub opcode: OpCode,
    pub is_authoritative_answer: bool,
    pub is_truncation: bool,
    pub is_recursion_desired: bool,
    pub is_recursion_available: bool,
    pub response_code: ResponseCode,
}

impl HeaderFlags {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        type ParsedFlags = (u8, u8, u8, u8, u8, u8, u8, u8);

        let (
            input,
            (
                qr,
                opcode,
                authoritative_answer,
                truncation,
                recursion_desired,
                recursion_available,
                _unused,
                response_code,
            ),
        ) = bits::<_, ParsedFlags, Error<(&[u8], usize)>, _, _>(tuple((
            take(1_usize),
            take(4_usize),
            take(1_usize),
            take(1_usize),
            take(1_usize),
            take(1_usize),
            take(3_usize),
            take(4_usize),
        )))(input)?;

        Ok((
            input,
            Self {
                qr: qr.into(),
                opcode: opcode.into(),
                is_authoritative_answer: authoritative_answer != 0,
                is_truncation: truncation != 0,
                is_recursion_desired: recursion_desired != 0,
                is_recursion_available: recursion_available != 0,
                response_code: response_code.into(),
            },
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum QrFlag {
    Query = 0,
    Reply = 1,
}

impl From<u8> for QrFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Query,
            _ => Self::Reply,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    Query = 0,
    InverseQuery = 1,
    Status = 2,
    #[default]
    Invalid = 0x0F, // Encoded on 4 bytes
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Query,
            1 => Self::InverseQuery,
            2 => Self::Status,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[repr(u8)]
pub enum ResponseCode {
    #[default]
    NoError = 0,
    FormatError = 1,
    ServerFail = 2,
    NonExistentDomain = 3,
    Invalid = 0x0F, // Encoded on 4 bytes
}

impl From<u8> for ResponseCode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NoError,
            1 => Self::FormatError,
            2 => Self::ServerFail,
            3 => Self::NonExistentDomain,
            _ => Self::Invalid,
        }
    }
}
