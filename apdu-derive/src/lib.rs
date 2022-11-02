//! Implementation of procedural macro for apdu crate.
//! By deriving apdu_derive::Response, you can convert from APDU raw response to an Enum easily.
//! Macro interface is inspired by thiserror crate.
//!
//! ## Examples
//! Here is a simple example to derive Response:
//! ```rust
//! #[derive(apdu_derive::Response)]
//! enum Response<'a> {
//!     #[apdu(0x90, 0x00)]
//!     Ok(&'a [u8]),
//!
//!     #[apdu(0x60..=0x69, _)]
//!     #[apdu(0x12, 0x34)]
//!     NotOk,
//!
//!     #[apdu(_, _)]
//!     Unknown(u8, u8),
//! }
//! ```
//!
//! This is equivalent to implementing this:
//! ```rust
//! enum Response<'a> {
//!     Ok(&'a [u8]),
//!     NotOk,
//!     Unknown(u8, u8),
//! }
//!
//! impl<'a> From<apdu_core::Response<'a>> for Response<'a> {
//!     fn from(response: apdu_core::Response<'a>) -> Self {
//!         match response.trailer {
//!             (0x90, 0x00) => Self::Ok(response.payload),
//!             (0x60..=0x69, _) => Self::NotOk,
//!             (_, _) => Self::Unknown(response.trailer.0, response.trailer.1),
//!         }
//!     }
//! }
//! ```
//!
//! Also you can combine them with thiserror derive:
//! ```rust
//! #[derive(Debug, apdu_derive::Response, thiserror::Error)]
//! enum Response {
//!     #[apdu(0x60..=0x69, _)]
//!     #[error("not ok!")]
//!     NotOk,
//!
//!     #[apdu(_, _)]
//!     #[error("unknown: {0:#X} {1:#X}")]
//!     Unknown(u8, u8),
//! }
//! ```
//!
//! Optionally you can select what to inject to the fields:
//! ```rust
//! #[derive(Debug, apdu_derive::Response, thiserror::Error)]
//! enum Response {
//!     #[apdu(0x63, 0xC0..=0xCF)]
//!     #[error("verify failed: {0} tries left")]
//!     VerifyFailed(#[sw2] #[mask(0x0F)] u8),
//!
//!     #[apdu(_, _)]
//!     #[error("unknown: {0:#X} {1:#X}")]
//!     Unknown(u8, u8),
//! }
//! ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Response, attributes(apdu, sw1, sw2, payload, mask))]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as DeriveInput);
    let output: proc_macro2::TokenStream = match item.data {
        Data::Enum(d) => {
            let ty = &item.ident;
            let gen = &item.generics;
            let gen_suffix = match gen.into_token_stream().is_empty() {
                true => quote! {},
                _ => quote! { ::#gen },
            };

            let arms = d
                .variants
                .iter()
                .flat_map(|variant| {
                    variant
                        .attrs
                        .iter()
                        .filter(|attr| attr.path.is_ident("apdu"))
                        .map(move |attr| (variant, attr))
                })
                .map(|(variant, attr)| {
                    let ident = &variant.ident;
                    let tokens = &attr.tokens;
                    let left = match tokens.is_empty() {
                        true => quote! { _ },
                        _ => tokens.clone(),
                    };

                    let fields = match &variant.fields {
                        Fields::Named(f) => f.named.iter().collect(),
                        Fields::Unnamed(f) => f.unnamed.iter().collect(),
                        Fields::Unit => vec![],
                    };

                    let right = if fields.iter().any(|f| !f.attrs.is_empty()) {
                        let values = fields.iter().map(|f| {
                            let mask = if let Some(attr) =
                                f.attrs.iter().find(|a| a.path.is_ident("mask"))
                            {
                                let m = &attr.tokens;

                                quote! { & #m }
                            } else {
                                quote! {}
                            };

                            if f.attrs.iter().any(|a| a.path.is_ident("sw1")) {
                                quote! { (response.trailer.0 #mask).into(), }
                            } else if f.attrs.iter().any(|a| a.path.is_ident("sw2")) {
                                quote! { (response.trailer.1 #mask).into(), }
                            } else if f.attrs.iter().any(|a| a.path.is_ident("payload")) {
                                quote! { response.payload.into(), }
                            } else {
                                quote! {}
                            }
                        });

                        quote! { #ty #gen_suffix::#ident(#(#values)*) }
                    } else if variant.fields.is_empty() {
                        quote! { #ty #gen_suffix::#ident }
                    } else if variant.fields.len() == 1 {
                        quote! { #ty #gen_suffix::#ident(response.payload) }
                    } else if variant.fields.len() == 2 {
                        quote! { #ty #gen_suffix::#ident(response.trailer.0, response.trailer.1) }
                    } else {
                        panic!("unsupported type of fields found")
                    };

                    quote! { #left => #right, }
                });

            quote! {
                impl<'a> ::std::convert::From<::apdu_core::Response<'a>> for #ty #gen {
                    fn from(response: ::apdu_core::Response<'a>) -> Self {
                        let (sw1, sw2) = response.trailer;

                        match (sw1, sw2) {
                            #(#arms)*
                        }
                    }
                }

                impl<'a> ::std::convert::From<::apdu_core::Error<'a>> for #ty #gen {
                    fn from(error: ::apdu_core::Error<'a>) -> Self {
                        error.response.into()
                    }
                }

                impl<'a> ::std::convert::From<&'a [u8]> for #ty #gen {
                    fn from(bytes: &'a [u8]) -> Self {
                        ::apdu_core::Response::from(bytes).into()
                    }
                }
            }
        }
        _ => panic!("deriving for Enum is only supported"),
    };

    proc_macro::TokenStream::from(output)
}
