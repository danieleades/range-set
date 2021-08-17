use std::{collections::BTreeSet, iter::FromIterator};
use step::Step;

use crate::{element::Element, storage::Storage};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Set<T>
where
    T: Ord + Clone + Step,
{
    storage: BTreeSet<Element<T>>,
}

impl<T> Set<T>
where
    T: Ord + Clone + Step,
{
    pub fn contains(&self, value: &T) -> bool {
        self.storage.contains_value(value)
    }

    /// Insert a new value into the set.
    ///
    /// Return true if the value was already present, or false if it was not
    pub fn insert(&mut self, value: T) -> bool {
        if self.contains(&value) {
            return true;
        }

        // There are 4 possible cases
        //
        // 1. the value is adjacent to the previous range but not to the next range
        // 2. the value is adjacent to the next range, but not the previous range
        // 3. the value is not adjacent to any ranges
        // 4. the value is adjacent to both the previous and the next range

        match (
            self.storage.take_prev_adjacent_element(&value),
            self.storage.take_next_adjacent_element(&value),
        ) {
            (None, None) => {
                self.storage.insert(value.into());
            }
            (None, Some(mut next)) => {
                next.extend_left_unchecked(value);
                self.storage.insert(next);
            }
            (Some(mut prev), None) => {
                prev.extend_right_unchecked(value);
                self.storage.insert(prev);
            }
            (Some(prev), Some(next)) => {
                let new_element = Element::new(prev.start().clone(), next.end().clone());
                self.storage.insert(new_element);
            }
        };

        false
    }

    pub fn into_tuples(self) -> impl Iterator<Item = (T, T)> {
        self.storage.into_iter().map(Into::into)
    }
}

impl<T> FromIterator<(T, T)> for Set<T>
where
    T: Ord + Step + Clone,
{
    fn from_iter<I: IntoIterator<Item = (T, T)>>(iter: I) -> Self {
        let storage = iter
            .into_iter()
            .map(|(start, end)| Element::new(start, end))
            .collect();
        Self { storage }
    }
}

#[cfg(test)]
mod tests {
    use super::Set;

    #[test]
    fn iter() {
        let mut set = Set::default();

        for x in [45_u32, 12_u32, 789_u32, 14_u32, 13_u32] {
            set.insert(x);
        }

        let tuples: Vec<(u32, u32)> = set.into_tuples().collect();

        assert_eq!(
            &tuples,
            &vec![(12_u32, 14_u32), (45_u32, 45_u32), (789_u32, 789_u32)]
        );

        let _new_set: Set<u32> = tuples.into_iter().collect();
    }
}
