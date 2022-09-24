//! Core library for composing APDU commands and parsing their responses.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod command;
mod error;
mod response;

pub use command::*;
pub use error::*;
pub use response::*;

/// An handler to handle an APDU command and receive a response in a specific context.
pub trait HandlerInCtx<Ctx = ()> {
    /// Handles the APDU command in a specific context.
    /// Implementations must transmit the command to the card through a reader,
    /// then receive the response from them.
    fn handle_in_ctx(&self, ctx: Ctx, command: Command) -> Response;
}

/// An handler to handle an APDU command and receive a response
pub trait Handler: HandlerInCtx<()> {
    /// Handles the APDU command.
    /// Implementations must transmit the command to the card through a reader,
    /// then receive the response from them.
    fn handle(&self, command: Command) -> Response {
        self.handle_in_ctx((), command)
    }
}
