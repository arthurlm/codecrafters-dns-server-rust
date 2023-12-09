use dns_starter_rust::{client::DnsClient, message::*};

#[test]
fn test_query() {
    let mut client = DnsClient::connect("0.0.0.0:2054", "8.8.8.8:53").unwrap();
    let answer = client
        .query(&QuestionSection::new_a("example.com"))
        .unwrap();

    assert_eq!(
        answer,
        AnswerSection {
            labels: vec!["example".to_string(), "com".to_string()],
            rr_type: ResourceRecordType::A,
            rr_class: ResourceRecordClass::IN,
            ttl: answer.ttl,
            // Yeah I know .. This IP may change and tests break, but that a good to start the client :)
            data: vec![93, 184, 216, 34],
        }
    );
}
