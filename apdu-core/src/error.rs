use core::fmt::{Display, Formatter};

use crate::Response;

/// An error that was returned from the card or reader
#[derive(Debug)]
pub struct Error<'a> {
    pub response: Response<'a>,
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let (sw1, sw2) = self.response.trailer;

        write!(f, "The APDU reader returned an error ({sw1:#X}, {sw2:#X}).")
    }
}

#[cfg(feature = "std")]
impl<'a> std::error::Error for Error<'a> {}
