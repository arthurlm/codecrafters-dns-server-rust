use std::io::{self, Write};

use nom::{number::complete::be_u16, IResult};

use super::{labels, ResourceRecordClass, ResourceRecordType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QuestionSection {
    pub labels: Vec<String>,
    pub label_offset: Option<u16>,
    pub rr_type: ResourceRecordType,
    pub rr_class: ResourceRecordClass,
}

impl QuestionSection {
    pub fn new_a(url: &str) -> Self {
        Self {
            labels: url.split('.').map(|x| x.to_string()).collect(),
            label_offset: None,
            rr_type: ResourceRecordType::A,
            rr_class: ResourceRecordClass::IN,
        }
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (labels, label_offset)) = labels::parse(input)?;
        let (input, rr_type_val) = be_u16(input)?;
        let (input, rr_class_val) = be_u16(input)?;

        Ok((
            input,
            Self {
                labels,
                label_offset,
                rr_type: rr_type_val.into(),
                rr_class: rr_class_val.into(),
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

        Ok(())
    }
}
