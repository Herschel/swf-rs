//! # swf-rs
//!
//! Library for reading and writing Adobe Flash SWF files.
//!
//! # Organization
//!
//! This library consits of a `read` module for decoding SWF data, and a `write` library for
//! writing SWF data.

extern crate byteorder;
extern crate flate2;
extern crate xz2;

mod read;
mod tag_codes;
mod types;
mod write;

/// Parses an SWF from a `Read` stream.
pub use read::read_swf;

/// Writes an SWF to a `Write` stream.
pub use write::write_swf;

/// Types used to represent a parsed SWF.
pub use types::*;