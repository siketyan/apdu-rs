//! Core library for composing APDU commands and parsing their responses.

#![deny(missing_debug_implementations)]
#![cfg_attr(not(feature = "std"), no_std)]

mod command;
mod error;
mod response;

pub use command::*;
pub use error::*;
pub use response::*;

use core::fmt::{Debug, Display, Formatter};

pub enum HandleError {
    /// The buffer is too small to write the response.
    /// Reallocate with the capacity and retry.
    NotEnoughBuffer(usize),

    /// Failed to communicate through physical NFC layer.
    /// Hardware or OS API error?
    Nfc(Box<dyn Display>),
}

impl HandleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use HandleError::*;
        match self {
            NotEnoughBuffer(size) => write!(
                f,
                "The buffer is too small to write the response. (needs {} bytes)",
                size,
            ),
            Nfc(e) => e.fmt(f),
        }
    }
}

impl Debug for HandleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Self::fmt(self, f)
    }
}

impl Display for HandleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Self::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HandleError {}

pub type Result = std::result::Result<usize, HandleError>;

/// An handler to handle an APDU command and receive a response in a specific context.
pub trait HandlerInCtx<Ctx = ()> {
    /// Handles the APDU command in a specific context.
    /// Implementations must transmit the command to the card through a reader,
    /// then receive the response from them, returning length of the data written.
    fn handle_in_ctx(&self, ctx: Ctx, command: &[u8], response: &mut [u8]) -> Result;
}

/// An handler to handle an APDU command and receive a response
pub trait Handler: HandlerInCtx<()> {
    /// Handles the APDU command.
    /// Implementations must transmit the command to the card through a reader,
    /// then receive the response from them, returning length of the data written.
    fn handle(&self, command: &[u8], response: &mut [u8]) -> Result {
        self.handle_in_ctx((), command, response)
    }
}
