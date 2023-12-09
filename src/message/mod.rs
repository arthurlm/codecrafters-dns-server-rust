use std::io::{self, Write};

use nom::{multi::count, IResult};

mod answer;
mod header;
mod question;
mod resource_record_class;
mod resource_record_type;

pub use answer::AnswerSection;
pub use header::*;
pub use question::QuestionSection;
pub use resource_record_class::ResourceRecordClass;
pub use resource_record_type::ResourceRecordType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<QuestionSection>,
    pub answers: Vec<AnswerSection>,
}

impl Message {
    pub fn parse(msg_input: &[u8]) -> IResult<&[u8], Self> {
        let (input, header) = Header::parse(msg_input)?;
        let (input, mut questions) =
            count(QuestionSection::parse, header.question_count as usize)(input)?;

        for question in &mut questions {
            question.resolve_offsets(msg_input)?;
        }

        Ok((
            input,
            Self {
                header,
                questions,
                answers: vec![],
            },
        ))
    }

    pub fn encode<W: Write>(&self, buf: &mut W) -> io::Result<()> {
        self.header.encode(buf)?;
        for question in &self.questions {
            question.encode(buf)?;
        }
        for answer in &self.answers {
            answer.encode(buf)?;
        }

        Ok(())
    }
}
