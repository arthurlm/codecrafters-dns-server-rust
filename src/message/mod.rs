use std::io::{self, Write};

use nom::{multi::count, IResult};

mod header;
mod question;
mod resource_record_class;
mod resource_record_type;

pub use header::*;
pub use question::QuestionSection;
pub use resource_record_class::ResourceRecordClass;
pub use resource_record_type::ResourceRecordType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<QuestionSection>,
}

impl Message {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, header) = Header::parse(input)?;
        let (input, questions) =
            count(QuestionSection::parse, header.question_count as usize)(input)?;

        Ok((input, Self { header, questions }))
    }

    pub fn encode<W: Write>(&self, buf: &mut W) -> io::Result<()> {
        self.header.encode(buf)?;
        for question in &self.questions {
            question.encode(buf)?;
        }

        Ok(())
    }
}
