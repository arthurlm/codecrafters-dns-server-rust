use nom::{
    bits::{bits, complete::take},
    branch::alt,
    bytes::complete::tag,
    combinator::{success, value, verify},
    error::Error,
    multi::{length_data, many_till},
    number::complete::be_u8,
    sequence::tuple,
    IResult, Parser,
};

/// Parse label output.
///
/// List of URL segment + optional offset in original message if data is compressed.
pub type ParseLabelOutput = (Vec<String>, Option<u16>);

pub fn parse(input: &[u8]) -> IResult<&[u8], ParseLabelOutput> {
    let (input, (names, (_, offset))) = many_till(
        // Map each "segment" into a string.
        length_data(be_u8).map(data_to_string),
        // Until:
        alt((
            // EOT.
            tuple((
                // Complete with empty value.
                success(()),
                // Return None as there is no next offset.
                value(None, tag("\0")),
            )),
            // Or split 16 next bytes into 2 + 14 bits.
            bits(tuple((
                // Discard first 2 bits.
                value(
                    (),
                    // Check if first 2 bits are 0b11.
                    verify(take::<_, u8, _, Error<_>>(2_usize), |v| *v == 0b11),
                ),
                // Keep remaining bits as "offset".
                take(14_usize).map(Some),
            ))),
        )),
    )(input)?;

    Ok((input, (names, offset)))
}

fn data_to_string(input: &[u8]) -> String {
    String::from_utf8_lossy(input).to_string()
}

pub fn resolve_offsets(input: &[u8], offset: Option<u16>) -> IResult<&[u8], Vec<String>> {
    match offset {
        None => Ok((input, vec![])),
        Some(idx) => {
            let (input, (next_labels, next_offset)) = parse(&input[idx as usize..])?;
            assert!(
                next_offset.is_none(),
                "Chaining offset in compressed message is not supported"
            );

            Ok((input, next_labels))
        }
    }
}
