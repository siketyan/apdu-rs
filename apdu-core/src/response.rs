use crate::Error;

/// An response that was received from the card
#[derive(Debug, Default)]
pub struct Response<'a> {
    pub payload: &'a [u8],
    pub trailer: (u8, u8),
}

impl<'a> Response<'a> {
    /// Creates an empty response.
    pub fn new() -> Self {
        Default::default()
    }

    /// Determines whether the response indicates success or not.
    pub fn is_ok(&self) -> bool {
        matches!(self.trailer, (0x90, 0x00) | (0x91, 0x00))
    }
}

impl<'a> From<&'a [u8]> for Response<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        let len = bytes.len();
        if len < 2 {
            return Self {
                payload: bytes,
                trailer: (0, 0),
            };
        }

        let sw2 = bytes[len - 1];
        let sw1 = bytes[len - 2];

        Self {
            payload: &bytes[..len - 2],
            trailer: (sw1, sw2),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> From<Response<'a>> for Result<&'a [u8], Error<'a>> {
    /// Converts the response to a result of octets.
    fn from(response: Response<'a>) -> Self {
        let is_ok = response.is_ok();

        match is_ok {
            true => Ok(response.payload),
            _ => Err(Error { response }),
        }
    }
}
