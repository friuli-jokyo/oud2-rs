//! # oud2

#![deny(missing_docs)]

pub mod de;
mod error;
pub mod ser;

pub use crate::de::{from_reader, from_slice, from_str, Deserializer};
pub use crate::ser::{to_string, to_vec, to_writer, Serializer};
