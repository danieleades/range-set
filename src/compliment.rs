use num_traits::Bounded;

pub struct Iter<I, T>
where
    I: Iterator<Item = (T, T)>,
    T: Bounded,
{
    iter: I,
    next_lower_bound: Option<T>,
}

impl<I, T> Iter<I, T>
where
    I: Iterator<Item = (T, T)>,
    T: Bounded + PartialEq,
{
    pub fn new(iter: I) -> Self {
        let next_lower_bound = Some(T::min_value());

        Self {
            iter,
            next_lower_bound,
        }
    }

    fn _next(&mut self) -> Option<(T, T)> {
        let start = self.next_lower_bound.take()?;

        let end = match self.iter.next() {
            Some((end, next_lower_bound)) => {
                self.next_lower_bound = Some(next_lower_bound);
                end
            }
            None => T::max_value(),
        };

        Some((start, end))
    }
}

impl<I, T> Iterator for Iter<I, T>
where
    I: Iterator<Item = (T, T)>,
    T: Bounded + PartialEq,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self._next() {
            Some(x) if x == (T::min_value(), T::min_value()) => self._next(),
            Some(x) if x == (T::max_value(), T::max_value()) => None,
            x => x,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Iter;
    use test_case::test_case;

    #[test_case(vec![(3, 5), (9, 12)] => vec![(0, 3), (5, 9), (12, u32::MAX)] ; "simple")]
    #[test_case(vec![(0, 5), (9, 12)] => vec![(5, 9), (12, u32::MAX)] ; "bounded by zero")]
    #[test_case(vec![(3, 5), (9, u32::MAX)] => vec![(0, 3), (5, 9)] ; "bounded by MAX")]
    fn compliment(input: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
        let iter = Iter::new(input.into_iter());
        iter.collect::<Vec<_>>()
    }
}
