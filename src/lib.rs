//! Type definitions for dealing with Starling webhook payloads.
//!
//! Strict types such as `Uuid` and `DateTime<Utc>` are used where possible, with custom enums used
//! for fields that have specific possible values.
//!
//! The Starling webhook documentation can be found
//! [here](https://developer.starlingbank.com/docs#api-reference-temp).

#[macro_use]
extern crate serde;

pub mod schema;

pub use schema::*;
