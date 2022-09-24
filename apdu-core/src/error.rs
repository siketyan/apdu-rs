use core::fmt::{Display, Formatter};

use crate::Response;

/// An error that was returned from the card or reader
#[derive(Debug)]
pub struct Error {
    pub response: Response,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let (sw1, sw2) = self.response.trailer;

        write!(
            f,
            "The APDU reader returned an error ({:#X}, {:#X}).",
            sw1, sw2,
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
