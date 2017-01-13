#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

/// Describes a method for adapting any iterator with the `Or` adapter.
pub trait OrIter
    where Self: Iterator + Sized
{
    /// Adapts an iterator using the `Or` iterator.
    ///
    /// In the event that the source iterator produces a `None` value without first having
    /// produced any other value, the `Or` iterator will produce a default value using the
    /// provided function. This default value will be produced once and only once.
    fn or<F: FnOnce() -> <Self as Iterator>::Item>(self, f: F) -> Or<Self, F>;
}

impl<I: Iterator> OrIter for I {
    fn or<F: FnOnce() -> <Self as Iterator>::Item>(self, f: F) -> Or<Self, F> {
        Or {
            source: self,
            default: Some(f),
            count: 0,
        }
    }
}

pub struct Or<I, F> {
    source: I,
    default: Option<F>,
    count: usize,
}

impl<I, F> Or<I, F>
    where I: Iterator,
          F: FnOnce() -> <I as Iterator>::Item
{
    fn default(&mut self) -> Option<<I as Iterator>::Item> {
        match self.count {
            0 if self.default.is_some() => self.default.take().map(|f| f()),
            _ => None,
        }
    }
}

impl<I, F> Iterator for Or<I, F>
    where I: Iterator,
          F: FnOnce() -> <I as Iterator>::Item
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            None => self.default(),
            Some(item) => {
                self.count += 1;
                Some(item)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::OrIter;
    use std::iter::Empty;

    #[test]
    fn empty_iterator_works() {
        let iterator = Empty::default().or(|| 3);
        let vec: Vec<_> = iterator.collect();

        assert_eq!(&[3], &vec[..]);
    }

    #[test]
    fn non_empty_iterator_works() {
        let iterator = (1..4).or(|| 0);
        let vec: Vec<_> = iterator.collect();

        assert_eq!(&[1, 2, 3], &vec[..]);
    }
}
