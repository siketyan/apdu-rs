# 🦀 apdu-rs
[![Rust](https://github.com/siketyan/apdu-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/siketyan/apdu-rs/actions/workflows/rust.yml)

Rust library to compose or parse APDU commands and responses.

## 🏗 Crates
This repository is made of these crates separately:

### apdu
[![crates.io](https://img.shields.io/crates/v/apdu.svg)](https://crates.io/crates/apdu)
[![docs](https://docs.rs/apdu/badge.svg)](https://docs.rs/apdu/)

### apdu-core
[![crates.io](https://img.shields.io/crates/v/apdu-core.svg)](https://crates.io/crates/apdu-core)
[![docs](https://docs.rs/apdu-core/badge.svg)](https://docs.rs/apdu-core/)

### apdu-derive
[![crates.io](https://img.shields.io/crates/v/apdu-derive.svg)](https://crates.io/crates/apdu-derive)
[![docs](https://docs.rs/apdu-derive/badge.svg)](https://docs.rs/apdu-derive/)

## 📦 Getting Started
Add to your Cargo.toml as a dependency as follows:
```toml
[dependencies]
apdu = "0.3"
```

## 🛠 Longer payloads support
This library supports longer payloads of APDU commands and responses.
If you want to use these, turn `longer_payloads` feature on:

```toml
apdu-core = { version = "0.3", features = ["longer_payloads"] }
```

## 🛠 no_std support
apdu-core crate does support no_std environments (but it requires `alloc` yet).
If you are using this crate in no_std, turn `std` feature off by disabling default features:

```toml
[dependencies]
apdu-core = { version = "0.3", default-features = false }
```

## 📄 Documentation
See [docs.rs](https://docs.rs/apdu/).
