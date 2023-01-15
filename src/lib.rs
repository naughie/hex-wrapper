//! `hex-wrapper` provides N-bit unsigned hexadecimal numbers.
//!
//! Roughly speaking, Hex converts between `uN` and (lowercase) [`String`] hexadecimally.
//!
//!
//! # Features
//!
//! Defaults are all off.
//!
//! - `rand`: Enables random constructors `HexN::rand()` or `HexN::with_rng()`.
//! - `serde`: Enables conversion used in [`serde`] (e.g. for json).
//! - `db`: Enables implementations of [`diesel`] v1.4 traits.
//!
//!
//! # Traits
//!
//! Each hex trait implements the following traits:
//!
//! - [`Default`] (only for `HexN`, not for `NonZeroHexN`);
//! - [`FromStr`](std::str::FromStr`);
//! - [`TryFrom<&str>`](`std::convert::TryFrom`);
//! - [`Debug`](`std::fmt::Debug`);
//! - [`Display`](`std::fmt::Display`) (hence [`ToString`] automatically);
//! - [`Into<String>`](`Into`) and conversely [`String`] implements [`From<HexN>`](`From`);
//! - [`Serialize`](`serde::Serialize`) and [`Deserialize`](`serde::Deserialize`) (only when the
//!   `serde` feature enabled).

#[macro_use]
mod macros;

mod zeroable;
pub use zeroable::*;

mod nonzero;
pub use nonzero::*;
