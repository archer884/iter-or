#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

use std::iter::{self, Chain, Once};

/// Allows an iterator to produce a default value in the event it produces nothing else.
pub trait IterOr
    where Self: Iterator + Sized
{
    /// Causes an iterator to produce a default item if empty.
    /// 
    /// In the event that the source iterator produces a `None` value without first having 
    /// produced any other value, the iterator will produce the default value supplied to 
    /// this function. This default value will be produced once and only once.
    fn or(mut self, item: Self::Item) -> Chain<Once<Self::Item>, Self> {
        iter::once(self.next().unwrap_or(item)).chain(self)
    }

    /// Cause an iterator to produce a default item if empty.
    ///
    /// In the event that the source iterator produces a `None` value without first having
    /// produced any other value, the iterator will produce a default value using the
    /// provided function. This default value will be produced once and only once.
    fn or_else<F: FnOnce() -> Self::Item>(mut self, f: F) -> Chain<Once<Self::Item>, Self> {
        iter::once(self.next().unwrap_or_else(f)).chain(self)
    }
}

impl<I: Iterator> IterOr for I { }

#[cfg(test)]
mod tests {
    use super::IterOr;
    use std::iter;

    #[test]
    fn empty_iterator_works() {
        let iterator = iter::empty().or_else(|| 3);
        let vec: Vec<_> = iterator.collect();

        assert_eq!(&[3], &vec[..]);
    }

    #[test]
    fn non_empty_iterator_works() {
        let iterator = (1..4).or_else(|| 0);
        let vec: Vec<_> = iterator.collect();

        assert_eq!(&[1, 2, 3], &vec[..]);
    }
}
