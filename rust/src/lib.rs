// This is free and unencumbered software released into the public domain.

//! This crate provides well-known countries.
//!
//! ```rust
//! use known_countries::Country;
//! ```

#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

//mod country;
//pub use country::*;

#[doc = include_str!("../../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
