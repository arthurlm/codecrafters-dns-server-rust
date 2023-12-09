use std::io::{self, Write};

use nom::{
    multi::length_data,
    number::complete::{be_u16, be_u32},
    AsBytes, IResult,
};

use super::{labels, ResourceRecordClass, ResourceRecordType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AnswerSection {
    pub labels: Vec<String>,
    pub label_offset: Option<u16>,
    pub rr_type: ResourceRecordType,
    pub rr_class: ResourceRecordClass,
    pub ttl: u32,
    pub data: Vec<u8>,
}

impl AnswerSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (labels, label_offset)) = labels::parse(input)?;
        let (input, rr_type_val) = be_u16(input)?;
        let (input, rr_class_val) = be_u16(input)?;
        let (input, ttl) = be_u32(input)?;
        let (input, data) = length_data(be_u16)(input)?;

        Ok((
            input,
            Self {
                labels,
                label_offset,
                rr_type: rr_type_val.into(),
                rr_class: rr_class_val.into(),
                ttl,
                data: data.to_vec(),
            },
        ))
    }

    pub fn resolve_offsets<'a>(&mut self, input: &'a [u8]) -> IResult<&'a [u8], ()> {
        if let Some(offset) = self.label_offset {
            let (_remaining, (next_labels, next_offset)) =
                labels::parse(&input[offset as usize..])?;
            assert!(
                next_offset.is_none(),
                "Chaining offset in compressed message is not supported"
            );

            self.labels.extend(next_labels);
            self.label_offset = None;
        }

        Ok((input, ()))
    }

    pub fn encode<W: Write>(&self, buf: &mut W) -> io::Result<()> {
        for label in &self.labels {
            // Write string len
            assert!(label.len() <= 0xFF, "Label '{}' is too long", label);
            buf.write_all(&[label.len() as u8])?;

            // Write string
            buf.write_all(label.as_bytes())?;
        }
        buf.write_all(&[0x00])?;

        // Write flags
        buf.write_all(&(self.rr_type as u16).to_be_bytes())?;
        buf.write_all(&(self.rr_class as u16).to_be_bytes())?;
        buf.write_all(&self.ttl.to_be_bytes())?;

        // Write data
        assert!(self.data.len() <= 0xFFFF, "Data is too long");
        buf.write_all(&(self.data.len() as u16).to_be_bytes())?;
        buf.write_all(self.data.as_bytes())?;

        Ok(())
    }
}
