//! An efficient set implementation for mostly contiguous elements

#![deny(clippy::all, missing_docs, missing_debug_implementations)]
#![warn(clippy::pedantic)]

mod compliment;
mod set;

pub use set::Set;
