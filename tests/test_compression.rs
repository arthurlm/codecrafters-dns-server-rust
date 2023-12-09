use dns_starter_rust::message::*;

#[test]
fn test_parse() {
    // Query to:
    // ;abc.longassdomainname.com.        IN       A
    // ;def.longassdomainname.com.        IN       A
    let input = [
        56, 58, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115, 115,
        100, 111, 109, 97, 105, 110, 110, 97, 109, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 3, 100,
        101, 102, 192, 16, 0, 1, 0, 1,
    ];

    let (_, msg) = Message::parse(&input).unwrap();
    assert_eq!(
        msg,
        Message {
            header: Header {
                id: 14394,
                flags: HeaderFlags {
                    qr: QrFlag::Query,
                    opcode: OpCode::Query,
                    is_authoritative_answer: false,
                    is_truncation: false,
                    is_recursion_desired: true,
                    is_recursion_available: false,
                    response_code: ResponseCode::NoError,
                },
                question_count: 2,
                answer_count: 0,
                authority_resource_record_count: 0,
                additional_resource_record_count: 0,
            },
            questions: vec![
                QuestionSection {
                    labels: vec![
                        "abc".to_string(),
                        "longassdomainname".to_string(),
                        "com".to_string()
                    ],
                    label_offset: None,
                    rr_type: ResourceRecordType::A,
                    rr_class: ResourceRecordClass::IN
                },
                QuestionSection {
                    labels: vec![
                        "def".to_string(),
                        "longassdomainname".to_string(),
                        "com".to_string()
                    ],
                    label_offset: None,
                    rr_type: ResourceRecordType::A,
                    rr_class: ResourceRecordClass::IN
                }
            ],
            answers: vec![],
        }
    )
}
