/// Record record type.
///
/// Check: https://www.rfc-editor.org/rfc/rfc1035#section-3.2.2
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ResourceRecordType {
    /// Invalid value.
    #[default]
    Invalid = 0,

    /// a host address.
    A = 1,
    /// an authoritative name server.
    NS = 2,
    /// a mail destination (Obsolete - use MX).
    MD = 3,
    /// a mail forwarder (Obsolete - use MX).
    MF = 4,
    /// the canonical name for an alias.
    CNAME = 5,
    /// marks the start of a zone of authority.
    SOA = 6,
    /// a mailbox domain name (EXPERIMENTAL).
    MB = 7,
    /// a mail group member (EXPERIMENTAL).
    MG = 8,
    /// a mail rename domain name (EXPERIMENTAL).
    MR = 9,
    /// a null RR (EXPERIMENTAL).
    NULL = 10,
    /// a well known service description.
    WKS = 11,
    /// a domain name pointer.
    PTR = 12,
    /// host information.
    HINFO = 13,
    /// mailbox or mail list information.
    MINFO = 14,
    /// mail exchange.
    MX = 15,
    /// text strings.
    TXT = 16,
}

impl From<u16> for ResourceRecordType {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::A,
            2 => Self::NS,
            3 => Self::MD,
            4 => Self::MF,
            5 => Self::CNAME,
            6 => Self::SOA,
            7 => Self::MB,
            8 => Self::MG,
            9 => Self::MR,
            10 => Self::NULL,
            11 => Self::WKS,
            12 => Self::PTR,
            13 => Self::HINFO,
            14 => Self::MINFO,
            15 => Self::MX,
            16 => Self::TXT,
            _ => Self::Invalid,
        }
    }
}
