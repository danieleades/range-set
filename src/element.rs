use std::ops::RangeInclusive;
use step::Step;

#[derive(Clone, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
pub struct Element<T>
where
    T: PartialOrd,
{
    range: RangeInclusive<T>,
}

impl<T> Element<T>
where
    T: PartialOrd + Clone + Step,
{
    pub fn new(start: T, end: T) -> Self {
        let range = RangeInclusive::new(start, end);
        Self { range }
    }

    pub fn start(&self) -> &T {
        self.range.start()
    }

    pub fn end(&self) -> &T {
        self.range.end()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.range.contains(value)
    }

    pub fn is_one_less_than(&self, value: &T) -> bool {
        match value.prev() {
            Some(prev) => self.contains(&prev),
            None => false,
        }
    }

    pub fn is_one_more_than(&self, value: &T) -> bool {
        match value.next() {
            Some(next) => self.contains(&next),
            None => false,
        }
    }

    pub fn extend_right_unchecked(&mut self, value: T) {
        debug_assert!(&value > self.end());
        let start = self.range.start();
        self.range = RangeInclusive::new(start.clone(), value);
    }

    pub fn extend_left_unchecked(&mut self, value: T) {
        debug_assert!(&value < self.start());
        let end = self.range.end();
        self.range = RangeInclusive::new(value, end.clone());
    }
}

impl<T> PartialOrd for Element<T>
where
    T: Ord + Clone + Step,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Element<T>
where
    T: Ord + Clone + Step,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start().cmp(other.start())
    }
}

impl<T> From<T> for Element<T>
where
    T: Ord + Clone,
{
    fn from(t: T) -> Self {
        Self {
            range: t.clone()..=t,
        }
    }
}

impl<T> From<Element<T>> for RangeInclusive<T>
where
    T: Ord + Clone,
{
    fn from(element: Element<T>) -> Self {
        element.range
    }
}

impl<T> From<RangeInclusive<T>> for Element<T>
where
    T: Ord + Clone,
{
    fn from(range: RangeInclusive<T>) -> Self {
        Self { range }
    }
}

impl<T> From<Element<T>> for (T, T)
where
    T: PartialOrd,
{
    fn from(element: Element<T>) -> Self {
        element.range.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use super::Element;
    use test_case::test_case;

    #[test]
    fn getters() {
        let element = Element::new(12_u32, 987_u32);
        assert_eq!(*element.start(), 12_u32);
        assert_eq!(*element.end(), 987_u32);
    }

    #[test]
    fn contains() {
        let element = Element::new(12_u32, 987_u32);
        assert!(element.contains(&100_u32));
        assert!(!element.contains(&10_u32));
    }

    #[test_case(4, 7, 10 ; "correct usage")]
    #[test_case(4, 7, 5 => panics ; "incorrect usage")]
    fn extend_right(start: u32, end: u32, extend: u32) {
        let mut element = Element::new(start, end);
        element.extend_right_unchecked(extend);
        assert_eq!(&start, element.start());
        assert_eq!(&extend, element.end());
    }

    #[test_case(4, 7, 2 ; "correct usage")]
    #[test_case(4, 7, 5 => panics ; "incorrect usage")]
    fn extend_left(start: u32, end: u32, extend: u32) {
        let mut element = Element::new(start, end);
        element.extend_left_unchecked(extend);
        assert_eq!(&extend, element.start());
        assert_eq!(&end, element.end());
    }

    #[test]
    fn conversions() {
        let _single_element = Element::from(100_u32);
        let element = Element::new(1_u32, 10_u32);
        let range: RangeInclusive<_> = element.into();
        let new_element: Element<_> = range.into();
        let tuple: (u32, u32) = new_element.into();
        let _new_new_element: Element<_> = tuple.into();
    }
}
