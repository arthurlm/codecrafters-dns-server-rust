use std::io::{self, Write};

use nom::AsBytes;

use super::{ResourceRecordClass, ResourceRecordType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AnswerSection {
    pub labels: Vec<String>,
    pub rr_type: ResourceRecordType,
    pub rr_class: ResourceRecordClass,
    pub ttl: u32,
    pub data: Vec<u8>,
}

impl AnswerSection {
    pub fn encode<W: Write>(&self, buf: &mut W) -> io::Result<()> {
        for label in &self.labels {
            // Write string len
            assert!(label.len() <= 0xFF, "Label '{}' is too long", label);
            buf.write(&[label.len() as u8])?;

            // Write string
            buf.write(label.as_bytes())?;
        }
        buf.write(&[0x00])?;

        // Write flags
        buf.write(&(self.rr_type as u16).to_be_bytes())?;
        buf.write(&(self.rr_class as u16).to_be_bytes())?;
        buf.write(&self.ttl.to_be_bytes())?;

        // Write data
        assert!(self.data.len() <= 0xFFFF, "Data is too long");
        buf.write(&(self.data.len() as u16).to_be_bytes())?;
        buf.write(self.data.as_bytes())?;

        Ok(())
    }
}
