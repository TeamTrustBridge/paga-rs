//! Async Rust SDK for the Paga Engine API.
//!
//! The examples below are the ones in the README, and are compiled as doc
//! tests, so they cannot drift out of date.
#![doc = include_str!("../README.md")]
#![cfg_attr(doc, allow(unused_imports))]

pub mod auth;
pub mod client;
pub mod codes;
pub mod errors;
pub mod requests;
pub mod types;
pub mod webhooks;

pub use auth::*;
pub use client::*;
pub use codes::*;
pub use errors::*;
pub use requests::*;
pub use types::*;
pub use webhooks::*;
