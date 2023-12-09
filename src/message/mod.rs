use std::io::{self, Write};

use nom::{multi::count, IResult};

mod answer;
mod header;
mod labels;
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
        // Parse msg
        let (input, header) = Header::parse(msg_input)?;
        let (input, questions_unresolved) =
            count(QuestionSection::parse, header.question_count as usize)(input)?;
        let (input, answers_unresolved) =
            count(AnswerSection::parse, header.answer_count as usize)(input)?;

        // Resolve compressed row
        let mut questions = Vec::with_capacity(questions_unresolved.len());
        for (mut question, offset) in questions_unresolved {
            let (_, next_labels) = labels::resolve_offsets(msg_input, offset)?;
            question.labels.extend(next_labels);
            questions.push(question);
        }

        let mut answers = Vec::with_capacity(answers_unresolved.len());
        for (mut answer, offset) in answers_unresolved {
            let (_, next_labels) = labels::resolve_offsets(msg_input, offset)?;
            answer.labels.extend(next_labels);
            answers.push(answer);
        }

        // Build response
        Ok((
            input,
            Self {
                header,
                questions,
                answers,
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
