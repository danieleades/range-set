use std::ops::Range;

mod btreemap;

pub trait Storage<T> {
    /// Check whether a value is already present
    fn contains(&self, value: &T) -> bool;

    /// Remove the adjacent range who's upper bound is less than the given value
    fn take_prev_range(&mut self, value: &T) -> Option<Range<T>>;

    /// Remove the adjacent range who's lower bound is greater than the given
    /// value and return it
    fn take_next_range(&mut self, value: &T) -> Option<Range<T>>;

    /// Insert a range into storage
    ///
    /// Inserting a range that already is already in storage is undefined (and
    /// should never be called), and implementations may panic.
    fn insert_range(&mut self, range: Range<T>);
}
