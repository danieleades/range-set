use num_traits::{Bounded, Num};
use std::{collections::BTreeMap, ops::Range};

use crate::compliment;

pub trait Element: Num + Clone + Ord + Bounded {}

impl<T> Element for T where T: Num + Clone + Ord + Bounded {}

/// A space-efficient set for mostly contiguous data
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Set<T>
where
    T: Element,
{
    storage: BTreeMap<T, T>,
}

impl<T> Set<T>
where
    T: Element,
{
    /// Check whether an element is contained within the set
    pub fn contains(&self, value: &T) -> bool {
        if self.storage.contains_key(value) {
            return true;
        }

        if let Some((_lower, upper)) = self.storage.range(..value).next_back() {
            if value < upper {
                return true;
            }
        }

        false
    }

    fn take_prev_range(&mut self, value: &T) -> Option<std::ops::Range<T>> {
        let key: T = self.storage.range(..value).next_back()?.0.clone();
        self.storage
            .remove_entry(&key)
            .map(|(start, end)| start..end)
    }

    fn take_next_range(&mut self, value: &T) -> Option<std::ops::Range<T>> {
        let key: T = self.storage.range(value..).next()?.0.clone();
        self.storage
            .remove_entry(&key)
            .map(|(start, end)| start..end)
    }

    fn insert_range(&mut self, range: std::ops::Range<T>) {
        if self.storage.insert(range.start, range.end).is_some() {
            panic!("range already present!");
        }
    }

    /// Insert a new element into the set
    pub fn insert(&mut self, element: T) {
        if self.contains(&element) {
            return;
        }

        let prev_range = self.take_prev_range(&element).map(|r| {
            let prev_adjacent = r.end == element;
            (r, prev_adjacent)
        });

        let next_range = self.take_next_range(&element).map(|r| {
            let next_adjacent = element.clone() + T::one() == r.start;
            (r, next_adjacent)
        });

        match (prev_range, next_range) {
            (None, None) => {
                self.insert_range(Range::from_value(element));
            }
            (None, Some((next_range, true))) => {
                self.insert_range(next_range.extend_lower());
            }
            (None, Some((next_range, false))) => {
                self.insert_range(next_range);
                self.insert_range(Range::from_value(element));
            }
            (Some((prev_range, true)), None) => {
                self.insert_range(prev_range.extend_upper());
            }
            (Some((prev_range, false)), None) => {
                self.insert_range(prev_range);
                self.insert_range(Range::from_value(element));
            }
            (Some((prev_range, false)), Some((next_range, false))) => {
                self.insert_range(Range::from_value(element));
                self.insert_range(prev_range);
                self.insert_range(next_range);
            }
            (Some((prev_range, false)), Some((next_range, true))) => {
                self.insert_range(prev_range);
                self.insert_range(next_range.extend_lower());
            }
            (Some((prev_range, true)), Some((next_range, false))) => {
                self.insert_range(prev_range.extend_upper());
                self.insert_range(next_range);
            }
            (Some((prev_range, true)), Some((next_range, true))) => {
                self.insert_range(prev_range.start..next_range.end);
            }
        }
    }

    /// Convert this set into its compliment
    #[must_use]
    pub fn into_compliment(self) -> Self {
        let storage = compliment::Iter::new(self.storage.into_iter()).collect();

        Self { storage }
    }
}

trait RangeExt<T> {
    fn from_value(value: T) -> Self;
    fn extend_upper(self) -> Self;
    fn extend_lower(self) -> Self;
}

impl<T> RangeExt<T> for Range<T>
where
    T: Num + Clone,
{
    fn from_value(value: T) -> Self {
        let end = value.clone() + T::one();
        value..end
    }

    fn extend_upper(mut self) -> Self {
        self.end = self.end + T::one();
        self
    }

    fn extend_lower(mut self) -> Self {
        self.start = self.start - T::one();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Set;
    use std::collections::BTreeMap;

    #[test]
    fn insert() {
        let mut set: Set<u32> = Set::default();
        set.insert(1);
        set.insert(3);
        set.insert(7);
        set.insert(5);
        set.insert(4);
        set.insert(4);

        let expected: BTreeMap<u32, u32> = [(1, 2), (3, 6), (7, 8)].iter().copied().collect();

        assert_eq!(set.storage, expected);
    }

    #[test]
    fn compliment_round_trip() {
        let mut expected: Set<u32> = Set::default();
        expected.insert(1);
        expected.insert(3);
        expected.insert(7);
        expected.insert(5);
        expected.insert(4);

        let set = expected.clone().into_compliment().into_compliment();

        assert_eq!(set, expected);
    }
}
