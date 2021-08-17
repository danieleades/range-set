use std::collections::BTreeSet;

use step::Step;

use crate::element::Element;

use super::Storage;

impl<T> Storage<T> for BTreeSet<Element<T>>
where
    T: Ord + Step + Clone,
{
    fn take(&mut self, element: &Element<T>) -> Option<Element<T>> {
        self.take(element)
    }

    fn prev_element(&self, value: &T) -> Option<&Element<T>> {
        let element = Element::from(value.clone());
        self.range(..element).next_back()
    }

    fn next_element(&self, value: &T) -> Option<&Element<T>> {
        let element = Element::from(value.clone());
        self.range(element..).next()
    }

    fn contains_value(&self, value: &T) -> bool {
        match self.next_element(value) {
            Some(element) => element.contains(value),
            None => false,
        }
    }

    fn insert(&mut self, element: Element<T>) -> bool {
        self.insert(element)
    }
}

#[cfg(test)]
mod tests {
    use crate::{element::Element, storage::Storage};
    use std::collections::BTreeSet;
    use test_case::test_case;

    #[test_case(&[1_u32, 3_u32], 2_u32 => (Some(1_u32), Some(3_u32)))]
    #[test_case(&[3_u32], 2_u32 => (None, Some(3_u32)))]
    #[test_case(&[1_u32, 2_u32], 3_u32 => (Some(2_u32), None))]
    #[test_case(&[1_u32, 2_u32], 2_u32 => (Some(1_u32), Some(2_u32)))]
    fn adjacent_elements(input: &[u32], value: u32) -> (Option<u32>, Option<u32>) {
        let mut storage = BTreeSet::default();
        for x in input.iter() {
            storage.insert(Element::from(*x));
        }

        let prev = storage.prev_element(&value).map(Element::start).copied();
        let next = storage.next_element(&value).map(Element::start).copied();

        (prev, next)
    }
}
