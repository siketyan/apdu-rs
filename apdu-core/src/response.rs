use crate::Error;

/// An response that was received from the card
#[derive(Default)]
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

impl From<Response> for Result<Vec<u8>, Error> {
    /// Converts the response to a result of octets.
    fn from(response: Response) -> Self {
        let is_ok = response.is_ok();
        let Response { payload, trailer } = response;

        match is_ok {
            true => Ok(payload),
            _ => Err(trailer.into()),
        }
    }
}

impl From<(u8, u8)> for Error {
    fn from((sw1, sw2): (u8, u8)) -> Self {
        Error { sw1, sw2 }
    }
}
