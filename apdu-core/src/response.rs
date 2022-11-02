use crate::Error;

/// An response that was received from the card
#[derive(Debug, Default)]
pub struct Response {
    pub payload: Vec<u8>,
    pub trailer: (u8, u8),
}

impl Response {
    /// Creates an empty response.
    pub fn new() -> Self {
        Default::default()
    }

    /// Determines whether the response indicates success or not.
    pub fn is_ok(&self) -> bool {
        matches!(self.trailer, (0x90, 0x00) | (0x91, 0x00))
    }
}

#[cfg(feature = "std")]
impl From<Vec<u8>> for Response {
    fn from(mut bytes: Vec<u8>) -> Self {
        let sw2 = bytes.pop();
        let sw1 = bytes.pop();

        Self {
            payload: bytes,
            trailer: match (sw1, sw2) {
                (Some(a), Some(b)) => (a, b),
                _ => (0x00, 0x00),
            },
        }
    }
}

#[cfg(feature = "std")]
impl From<Response> for Result<Vec<u8>, Error> {
    /// Converts the response to a result of octets.
    fn from(response: Response) -> Self {
        let is_ok = response.is_ok();

        match is_ok {
            true => Ok(response.payload),
            _ => Err(Error { response }),
        }
    }
}
