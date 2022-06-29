//! High-level API for APDU commands and responses.
//!
//! ## Features
//! ### Low-level APDU types
//! [apdu-core](https://docs.rs/apdu-core/) crate declares types for APDU command and response in low-level.
//! It is fully cross-platform since this crate contains only type declarations.
//! ```rust
//! let command = apdu_core::Command::new_with_payload(0x00, 0xA4, 0x12, 0x34, vec![0x56, 0x78]);
//! let bytes: Vec<u8> = command.into();
//!
//! assert_eq!(
//!     vec![
//!         0x00, 0xA4, 0x12, 0x34, // CLA + INS + P1 + P2 (required),
//!         0x02,                   // Lc, automatically calculated from Vec,
//!         0x56, 0x78,             // ...and the command payload.
//!     ],
//!     bytes,
//! );
//! ```
//!
//! ### High-level APIs
//! This apdu crate declares some high-level APIs to compose APDU commands or parse their responses easily.
//! It is cross-platform now, but some os-specific features can be added in the future.
//! ```rust
//! let command = apdu::command::select_file(0x12, 0x34, vec![0x56, 0x78]);
//! let bytes: Vec<u8> = command.into();
//!
//! assert_eq!(vec![0x00, 0xA4, 0x12, 0x34, 0x02, 0x56, 0x78], bytes);
//! ```
//!
//! Collection of APDU commands are incomplete and still in development.
//! We are welcome for your contribution at any time :)
//!
//! ### Abstraction
//! Among crates that supports communicating using APDU, it can use apdu-core crate for abstraction.
//! For example:
//! ```rust
//! /// You have an command that can be transmitted as APDU:
//! struct DoSomethingCommand {
//!     parameters: Vec<String>,
//! }
//!
//! /// Now implement `From<YourType>` for `apdu_core::Command`:
//! impl From<DoSomethingCommand> for apdu_core::Command {
//!     fn from(cmd: DoSomethingCommand) -> Self {
//!         Self::new(0x12, 0x34, 0x56, 0x78)
//!     }
//! }
//!
//! /// ... then dependents of your library can be used with other crate that has an APDU implementation:
//! fn handle_apdu_command(cmd: impl Into<apdu_core::Command>) {
//!     // connect to your card ...
//!     // transmit the command ...
//!     // ... and wait for the response
//! }
//! ```
//!
//! For advance, this crate also supports abstraction of APDU transmitter (called Handler in apdu_core).
//! `handle_apdu_command` from the above example can be transformed to:
//! ```rust
//! struct NfcReader;
//!
//! impl apdu_core::HandlerInCtx<()> for NfcReader {
//!     fn handle_in_ctx(&self, _ctx: (), cmd: apdu_core::Command) -> apdu_core::Response {
//!         // connect to your card ...
//!         // transmit the command ...
//!         // ... and wait for the response
//!
//!         vec![].into()
//!     }
//! }
//! ```
//!
//! Thanks to this abstraction, application developer can choose how the APDU command is transmitted
//! to the card independently to their payload. This enables you to implement your libraries
//! that uses APDU in cross-platform easily!
//!
//! ## Examples
//! [jpki-rs](https://github.com/siketyan/jpki-rs) is implemented using this apdu crate.
//! Take a look for catch example usages for this crate.

pub mod command;

pub use apdu_core as core;

/// Procedural macro to derive APDU response. See [apdu-derive](https://docs.rs/apdu-derive/) for details.
pub use apdu_derive::Response;

pub use crate::core::{Command, Handler, Response};

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
