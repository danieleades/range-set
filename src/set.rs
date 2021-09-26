use crate::storage::Storage;
use std::{collections::BTreeMap, marker::PhantomData, ops::Range};
use step::Step;

/// A space-efficient set for mostly contiguous data
#[derive(Debug)]
pub struct Set<T, S: Storage<T> = BTreeMap<T, T>> {
    storage: S,
    t: PhantomData<T>,
}

impl<T> Default for Set<T, BTreeMap<T, T>>
where
    T: Ord + Clone,
{
    fn default() -> Self {
        let storage = BTreeMap::default();
        let t = PhantomData;

        Self { storage, t }
    }
}

impl<T, S: Storage<T>> Set<T, S>
where
    T: PartialEq + Step,
{
    /// Check whether an element is contained within the set
    pub fn contains(&self, element: &T) -> bool {
        self.storage.contains(element)
    }

    /// Insert a new element into the set
    pub fn insert(&mut self, element: T) {
        let prev_range = self.storage.take_prev_range(&element).map(|r| {
            let prev_adjacent = r.end == element;
            (r, prev_adjacent)
        });

        let next_range = self.storage.take_next_range(&element).map(|r| {
            let next_adjacent = element.next().as_ref() == Some(&r.start);
            (r, next_adjacent)
        });

        match (prev_range, next_range) {
            (None, None) => {
                self.storage.insert_range(Range::from_value(element));
            }
            (None, Some((next_range, true))) => {
                self.storage.insert_range(next_range.extend_lower());
            }
            (None, Some((next_range, false))) => {
                self.storage.insert_range(next_range);
                self.storage.insert_range(Range::from_value(element));
            }
            (Some((prev_range, true)), None) => {
                self.storage.insert_range(prev_range.extend_upper());
            }
            (Some((prev_range, false)), None) => {
                self.storage.insert_range(prev_range);
                self.storage.insert_range(Range::from_value(element));
            }
            (Some((prev_range, false)), Some((next_range, false))) => {
                self.storage.insert_range(Range::from_value(element));
                self.storage.insert_range(prev_range);
                self.storage.insert_range(next_range);
            }
            (Some((prev_range, false)), Some((next_range, true))) => {
                self.storage.insert_range(prev_range);
                self.storage.insert_range(next_range.extend_lower());
            }
            (Some((prev_range, true)), Some((next_range, false))) => {
                self.storage.insert_range(prev_range.extend_upper());
                self.storage.insert_range(next_range);
            }
            (Some((prev_range, true)), Some((next_range, true))) => {
                self.storage.insert_range(prev_range.start..next_range.end);
            }
        }
    }
}

trait RangeExt<T> {
    fn from_value(value: T) -> Self;
    fn extend_upper(self) -> Self;
    fn extend_lower(self) -> Self;
}

impl<T> RangeExt<T> for Range<T>
where
    T: Step,
{
    fn from_value(value: T) -> Self {
        let end = value.next().unwrap();
        value..end
    }

    fn extend_upper(mut self) -> Self {
        self.end = self.end.next().unwrap();
        self
    }

    fn extend_lower(mut self) -> Self {
        self.start = self.start.prev().unwrap();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Set;
    use std::collections::BTreeMap;

    #[test]
    fn btreemap() {
        let mut set: Set<u32> = Set::default();
        set.insert(1);
        set.insert(3);
        set.insert(7);
        set.insert(5);
        set.insert(4);

        let expected: BTreeMap<u32, u32> = [(1, 2), (3, 6), (7, 8)].iter().copied().collect();

        assert_eq!(set.storage, expected);
    }
}
