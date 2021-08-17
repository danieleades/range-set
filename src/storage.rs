use crate::element::Element;
use step::Step;

mod btreeset;

pub trait Storage<T>
where
    T: PartialOrd + Clone + Step,
{
    fn insert(&mut self, element: Element<T>) -> bool;
    fn take(&mut self, element: &Element<T>) -> Option<Element<T>>;
    fn prev_element(&self, value: &T) -> Option<&Element<T>>;
    fn next_element(&self, value: &T) -> Option<&Element<T>>;

    fn prev_adjacent_element(&self, value: &T) -> Option<&Element<T>> {
        if let Some(element) = self.prev_element(value) {
            if element.is_one_less_than(value) {
                return Some(element);
            }
        }

        None
    }

    fn next_adjacent_element(&self, value: &T) -> Option<&Element<T>> {
        if let Some(element) = self.next_element(value) {
            if element.is_one_more_than(value) {
                return Some(element);
            }
        }

        None
    }

    fn take_prev_adjacent_element(&mut self, value: &T) -> Option<Element<T>> {
        self.prev_adjacent_element(value)
            .cloned()
            .map(|e| self.take(&e).unwrap())
    }

    fn take_next_adjacent_element(&mut self, value: &T) -> Option<Element<T>> {
        self.next_adjacent_element(value)
            .cloned()
            .map(|e| self.take(&e).unwrap())
    }

    fn contains_value(&self, value: &T) -> bool;
}
