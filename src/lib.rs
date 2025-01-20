//! # oud2

#![deny(missing_docs)]

mod error;
pub mod ser;

pub use crate::ser::{to_string, to_vec, to_writer, Serializer};
