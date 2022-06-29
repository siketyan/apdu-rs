//! High-level API for APDU commands and responses.

pub use apdu_core as core;
pub use apdu_derive::Response;

pub use crate::core::Command;
pub use crate::core::Response;

#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Eq, crate::Response)]
    enum Response {
        #[apdu(0x90, 0x00)]
        Ok(Vec<u8>),

        #[apdu(0x60..=0x69, _)]
        NotOk,

        #[apdu(_, _)]
        Unknown(u8, u8),
    }

    #[test]
    fn test_success() {
        let bytes: Vec<u8> = vec![0x12, 0x34, 0x56, 0x90, 0x00];
        let response = Response::from(bytes.clone());

        if let Response::Ok(payload) = response {
            assert_eq!(&bytes[..3], &payload)
        } else {
            panic!("Response is not Ok variant")
        }
    }

    #[test]
    fn test_not_ok() {
        let bytes: Vec<u8> = vec![0x69, 0x12];
        let response = Response::from(bytes.clone());

        assert_eq!(Response::NotOk, response)
    }

    #[test]
    fn test_unknown() {
        let bytes: Vec<u8> = vec![0x70, 0x00];
        let response = Response::from(bytes.clone());

        if let Response::Unknown(sw1, sw2) = response {
            assert_eq!((0x70, 0x00), (sw1, sw2))
        } else {
            panic!("Response is not Unknown variant")
        }
    }
}
