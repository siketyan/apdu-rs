use std::fmt::{Display, Formatter};

/// An error that was returned from the card or reader
#[derive(Debug)]
pub struct Error {
    pub sw1: u8,
    pub sw2: u8,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The APDU reader returned an error ({:#X}, {:#X}).",
            self.sw1, self.sw2
        )
    }
}

impl std::error::Error for Error {}
