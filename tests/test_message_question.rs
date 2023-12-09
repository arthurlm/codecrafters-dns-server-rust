use dns_starter_rust::message::*;

#[test]
fn test_parse_empty() {
    let input = b"\x00\x00\x01\x00\x01";

    // Test parse
    let (_, q) = QuestionSection::parse(input).unwrap();
    assert_eq!(
        q,
        QuestionSection {
            labels: vec![],
            label_offset: None,
            rr_type: ResourceRecordType::A,
            rr_class: ResourceRecordClass::IN,
        }
    );

    // Test encode
    let mut buf = Vec::with_capacity(input.len());
    q.encode(&mut buf).unwrap();
    assert_eq!(buf, input);
}

#[test]
fn test_parse_full() {
    let input = b"\x06google\x03com\x00\x00\x09\x00\x02";

    // Test parse
    let (_, q) = QuestionSection::parse(input).unwrap();
    assert_eq!(
        q,
        QuestionSection {
            labels: vec!["google".to_string(), "com".to_string()],
            label_offset: None,
            rr_type: ResourceRecordType::MR,
            rr_class: ResourceRecordClass::CS,
        }
    );

    // Test encode
    let mut buf = Vec::with_capacity(input.len());
    q.encode(&mut buf).unwrap();
    assert_eq!(buf, input);
}
