use std::collections::BTreeMap;

use super::Storage;

impl<T> Storage<T> for BTreeMap<T, T>
where
    T: Ord + Clone,
{
    fn contains(&self, value: &T) -> bool {
        if self.contains_key(value) {
            return true;
        }

        if let Some((_lower, upper)) = self.range(..value).next_back() {
            if value < upper {
                return true;
            }
        }

        false
    }

    fn take_prev_range(&mut self, value: &T) -> Option<std::ops::Range<T>> {
        let key: T = self.range(..value).next_back()?.0.clone();
        self.remove_entry(&key).map(|(start, end)| start..end)
    }

    fn take_next_range(&mut self, value: &T) -> Option<std::ops::Range<T>> {
        let key: T = self.range(value..).next()?.0.clone();
        self.remove_entry(&key).map(|(start, end)| start..end)
    }

    fn insert_range(&mut self, range: std::ops::Range<T>) {
        if self.insert(range.start, range.end).is_some() {
            panic!("range already present!");
        }
    }
}

trait BTreeMapExt<T> {}

impl<T> BTreeMapExt<T> for BTreeMap<T, T> {}

#[cfg(test)]
mod tests {
    use super::Storage;
    use std::collections::BTreeMap;

    #[test]
    fn contains() {
        let input = [(1, 2), (5, 10)];
        let map: BTreeMap<i32, i32> = input.iter().copied().collect();

        assert!(map.contains(&1));
        assert!(!map.contains(&2));
        assert!(!map.contains(&4));
        assert!(map.contains(&5));
        assert!(map.contains(&9));
        assert!(!map.contains(&10));
    }
}
