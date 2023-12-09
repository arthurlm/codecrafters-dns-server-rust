use dns_starter_rust::message::*;

#[test]
fn test_parse_header() {
    const INPUT: [u8; 12] = [
        0x12,
        0x34,
        0b1001_0111,
        0b1000_0000,
        0,
        42,
        0,
        56,
        0,
        12,
        0,
        38,
    ];

    // Test parse
    let (_, h) = Header::parse(&INPUT).unwrap();
    assert_eq!(
        h,
        Header {
            id: 0x1234,
            flags: HeaderFlags {
                qr: QrFlag::Reply,
                opcode: OpCode::Status,
                is_authoritative_answer: true,
                is_truncation: true,
                is_recursion_desired: true,
                is_recursion_available: true,
                response_code: ResponseCode::NoError,
            },
            question_count: 42,
            answer_count: 56,
            authority_resource_record_count: 12,
            additional_resource_record_count: 38,
        }
    );

    // Test encode
    let mut buf = Vec::with_capacity(INPUT.len());
    h.encode(&mut buf).unwrap();
    assert_eq!(buf, INPUT);
}

#[test]
fn test_parse_header_flags_qr() {
    let (_, h) = HeaderFlags::parse(&[0b0000_0000, 0b0000_0000]).unwrap();
    assert_eq!(
        h,
        HeaderFlags {
            qr: QrFlag::Query,
            opcode: OpCode::Query,
            is_authoritative_answer: false,
            is_truncation: false,
            is_recursion_desired: false,
            is_recursion_available: false,
            response_code: ResponseCode::NoError,
        }
    );

    let (_, h) = HeaderFlags::parse(&[0b1000_0000, 0b0000_0000]).unwrap();
    assert_eq!(
        h,
        HeaderFlags {
            qr: QrFlag::Reply,
            opcode: OpCode::Query,
            is_authoritative_answer: false,
            is_truncation: false,
            is_recursion_desired: false,
            is_recursion_available: false,
            response_code: ResponseCode::NoError,
        }
    );
}

#[test]
fn test_parse_header_flags_opcode() {
    let (_, h) = HeaderFlags::parse(&[0b0000_1000, 0b0000_0000]).unwrap();
    assert_eq!(
        h,
        HeaderFlags {
            qr: QrFlag::Query,
            opcode: OpCode::InverseQuery,
            is_authoritative_answer: false,
            is_truncation: false,
            is_recursion_desired: false,
            is_recursion_available: false,
            response_code: ResponseCode::NoError,
        }
    );

    let (_, h) = HeaderFlags::parse(&[0b0100_0000, 0b0000_0000]).unwrap();
    assert_eq!(
        h,
        HeaderFlags {
            qr: QrFlag::Query,
            opcode: OpCode::Invalid,
            is_authoritative_answer: false,
            is_truncation: false,
            is_recursion_desired: false,
            is_recursion_available: false,
            response_code: ResponseCode::NoError,
        }
    );
}

#[test]
fn test_parse_header_flags_bool() {
    let (_, h) = HeaderFlags::parse(&[0b0000_0101, 0b0000_0000]).unwrap();
    assert_eq!(
        h,
        HeaderFlags {
            qr: QrFlag::Query,
            opcode: OpCode::Query,
            is_authoritative_answer: true,
            is_truncation: false,
            is_recursion_desired: true,
            is_recursion_available: false,
            response_code: ResponseCode::NoError,
        }
    );

    let (_, h) = HeaderFlags::parse(&[0b0000_0010, 0b1000_0000]).unwrap();
    assert_eq!(
        h,
        HeaderFlags {
            qr: QrFlag::Query,
            opcode: OpCode::Query,
            is_authoritative_answer: false,
            is_truncation: true,
            is_recursion_desired: false,
            is_recursion_available: true,
            response_code: ResponseCode::NoError,
        }
    );
}

#[test]
fn test_parse_header_flags_response_code() {
    let (_, h) = HeaderFlags::parse(&[0b0000_0000, 0b0000_0010]).unwrap();
    assert_eq!(
        h,
        HeaderFlags {
            qr: QrFlag::Query,
            opcode: OpCode::Query,
            is_authoritative_answer: false,
            is_truncation: false,
            is_recursion_desired: false,
            is_recursion_available: false,
            response_code: ResponseCode::ServerFail,
        }
    );
}
