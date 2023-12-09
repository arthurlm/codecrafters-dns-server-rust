use std::io::{self, Write};

use nom::{
    bits::{bits, complete::take},
    branch::alt,
    bytes::complete::tag,
    combinator::{value, verify},
    error::{Error, ErrorKind},
    multi::{length_data, many_till},
    number::complete::{be_u16, be_u8},
    sequence::tuple,
    Err, IResult, Parser,
};

use super::{ResourceRecordClass, ResourceRecordType};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct QuestionSection {
    pub labels: Vec<String>,
    pub label_offset: Option<u16>,
    pub rr_type: ResourceRecordType,
    pub rr_class: ResourceRecordClass,
}

impl QuestionSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, (labels, label_offset)) = parse_label(input)?;
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
            let (_remaining, (next_labels, next_offset)) = parse_label(&input[offset as usize..])?;
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

fn data_to_string(input: &[u8]) -> String {
    String::from_utf8_lossy(input).to_string()
}

/// Parse label output.
///
/// List of URL segment + optional offset in original message if data is compressed.
type ParseLabelOutput = (Vec<String>, Option<u16>);

fn parse_label(input: &[u8]) -> IResult<&[u8], ParseLabelOutput> {
    alt((parse_label_regular, parse_label_compressed))(input)
}

fn parse_label_regular(input: &[u8]) -> IResult<&[u8], ParseLabelOutput> {
    match many_till(
        // Map each "segment" into a string.
        length_data(be_u8).map(data_to_string),
        // Until = EOT.
        tag("\0"),
    )(input)
    // Convert error as recoverable error so alt we try to parse the data.
    {
        Ok((input, (names, _))) => Ok((input, (names, None))),
        Err(Err::Failure(e) | Err::Error(e)) => Err(Err::Error(e)),
        Err(Err::Incomplete(_)) => Err(Err::Error(Error::new(input, ErrorKind::Alt))),
    }
}

fn parse_label_compressed(input: &[u8]) -> IResult<&[u8], ParseLabelOutput> {
    let (input, (names, (_, offset))) = many_till(
        // Map each "segment" into a string.
        length_data(be_u8).map(data_to_string),
        // until = split 16 next bytes into 2 + 14 bits.
        bits(tuple((
            // Discard first 2 bits.
            value(
                (),
                // Check if first 2 bits are 0b11.
                verify(take::<_, u8, _, Error<_>>(2_usize), |v| *v == 0b11),
            ),
            // Keep remaining bits as "offset".
            take::<_, u16, _, Error<_>>(14_usize),
        ))),
    )(input)?;

    Ok((input, (names, Some(offset))))
}
