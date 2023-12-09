use std::io::{self, Write};

use nom::{number::complete::be_u16, IResult};

use super::{labels, ResourceRecordClass, ResourceRecordType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QuestionSection {
    pub labels: Vec<String>,
    pub rr_type: ResourceRecordType,
    pub rr_class: ResourceRecordClass,
}

/// Parsed question: data + offset of compressed data (if compression is enabled)
pub type ParsedQuestion = (QuestionSection, Option<u16>);

impl QuestionSection {
    pub fn new_a(url: &str) -> Self {
        Self {
            labels: url.split('.').map(|x| x.to_string()).collect(),
            rr_type: ResourceRecordType::A,
            rr_class: ResourceRecordClass::IN,
        }
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], ParsedQuestion> {
        let (input, (labels, label_offset)) = labels::parse(input)?;
        let (input, rr_type_val) = be_u16(input)?;
        let (input, rr_class_val) = be_u16(input)?;

        Ok((
            input,
            (
                Self {
                    labels,

                    rr_type: rr_type_val.into(),
                    rr_class: rr_class_val.into(),
                },
                label_offset,
            ),
        ))
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

        Ok(())
    }
}
