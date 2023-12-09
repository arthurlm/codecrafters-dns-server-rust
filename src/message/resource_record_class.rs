/// Record record class.
///
/// Check: https://www.rfc-editor.org/rfc/rfc1035#section-3.2.4
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ResourceRecordClass {
    /// Invalid value.
    #[default]
    Invalid = 0,

    /// the Internet
    IN = 1,
    /// the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CS = 2,
    /// the CHAOS class
    CH = 3,
    /// Hesiod [Dyer 87]
    HS = 4,
}

impl From<u16> for ResourceRecordClass {
    fn from(value: u16) -> Self {
        match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            _ => Self::Invalid,
        }
    }
}
