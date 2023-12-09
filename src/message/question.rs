use std::io::{self, Write};

use nom::{
    bytes::complete::tag,
    multi::{length_data, many_till},
    number::complete::{be_u16, be_u8},
    IResult,
};

use super::{ResourceRecordClass, ResourceRecordType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QuestionSection {
    pub labels: Vec<String>,
    pub rr_type: ResourceRecordType,
    pub rr_class: ResourceRecordClass,
}

impl QuestionSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (names, _)) = many_till(length_data(be_u8), tag("\0"))(input)?;
        let (input, rr_type_val) = be_u16(input)?;
        let (input, rr_class_val) = be_u16(input)?;

        Ok((
            input,
            Self {
                labels: names
                    .into_iter()
                    .map(|x| String::from_utf8_lossy(x).to_string())
                    .collect(),
                rr_type: rr_type_val.into(),
                rr_class: rr_class_val.into(),
            },
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
