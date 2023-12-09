use std::io::{self, Write};

use nom::{multi::count, IResult};

use self::{header::Header, question::QuestionSection};

pub mod header;
pub mod question;
pub mod resource_record_class;
pub mod resource_record_type;

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
