use nom::{
    bits::{bits, complete::take},
    branch::alt,
    bytes::complete::tag,
    combinator::{value, verify},
    error::{Error, ErrorKind},
    multi::{length_data, many_till},
    number::complete::be_u8,
    sequence::tuple,
    Err, IResult, Parser,
};

/// Parse label output.
///
/// List of URL segment + optional offset in original message if data is compressed.
pub type ParseLabelOutput = (Vec<String>, Option<u16>);

pub fn parse(input: &[u8]) -> IResult<&[u8], ParseLabelOutput> {
    alt((parse_regular, parse_compressed))(input)
}

fn parse_regular(input: &[u8]) -> IResult<&[u8], ParseLabelOutput> {
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

fn parse_compressed(input: &[u8]) -> IResult<&[u8], ParseLabelOutput> {
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

fn data_to_string(input: &[u8]) -> String {
    String::from_utf8_lossy(input).to_string()
}
