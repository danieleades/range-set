#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod element;
mod set;
mod storage;

pub use self::set::Set as RangeSet;
