use num_traits::{Bounded, Num};
use std::{collections::BTreeMap, ops::Range};

use crate::compliment;

pub trait Element: Num + Clone + Ord {}

impl<T> Element for T where T: Num + Clone + Ord {}

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

        if let Some(range) = self.prev_range(value) {
            if value < range.end {
                return true;
            }
        }

        false
    }

    fn prev_range(&self, value: &T) -> Option<Range<&T>> {
        self.storage
            .range(..value)
            .next_back()
            .map(|(start, end)| start..end)
    }

    fn next_range(&self, value: &T) -> Option<Range<&T>> {
        self.storage
            .range(value..)
            .next()
            .map(|(start, end)| start..end)
    }

    /// Insert a new element into the set
    pub fn insert(&mut self, element: T) {
        #![allow(clippy::missing_panics_doc)]

        if self.contains(&element) {
            return;
        }

        let prev_adjacent_range = self
            .prev_range(&element)
            .filter(|range| range.end == &element);

        let next_adjacent_range = self
            .next_range(&element)
            .filter(|range| &(element.clone() + T::one()) == range.start);

        let operation = match (prev_adjacent_range, next_adjacent_range) {
            (None, None) => Operation::Insert(element),
            (None, Some(next_range)) => Operation::ExtendLower(next_range.start.clone()),
            (Some(prev_range), None) => Operation::ExtendUpper(prev_range.start.clone()),
            (Some(prev_range), Some(next_range)) => {
                Operation::Merge(prev_range.start.clone(), next_range.start.clone())
            }
        };

        match operation {
            Operation::ExtendUpper(idx) => {
                let upper_bound = self.storage.remove(&idx).unwrap();
                let lower_bound = idx - T::one();
                self.storage.insert(lower_bound, upper_bound);
            }
            Operation::ExtendLower(idx) => {
                let upper_bound = self.storage.get_mut(&idx).unwrap();
                *upper_bound = upper_bound.clone() + T::one();
            }
            Operation::Insert(e) => {
                let end = e.clone() + T::one();
                self.storage.insert(e, end);
            }
            Operation::Merge(prev, next) => {
                let upper_bound = self.storage.remove(&next).unwrap();
                *self.storage.get_mut(&prev).unwrap() = upper_bound;
            }
        }
    }
}

impl<T> Set<T>
where
    T: Element + Bounded,
{
    /// Convert this set into its compliment
    #[must_use]
    pub fn into_compliment(self) -> Self {
        let storage = compliment::Iter::new(self.storage.into_iter()).collect();

        Self { storage }
    }
}

pub enum Operation<T> {
    ExtendUpper(T),
    ExtendLower(T),
    Insert(T),
    Merge(T, T),
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
        set.insert(1);

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
