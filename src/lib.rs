#![deny(unused, missing_docs)]
//! `MergeIter` is an iterator that returns the elements of two iterators in order, given both
//! iterators are sorted.
//!
//! **Important note**: This iterator only works as intended, if both input iterators are sorted.
//! There are no checks in place to validate this property.

#[cfg(test)]
#[macro_use]
extern crate proptest;

/// A sorted iterator over two independent sorted iterators.
pub struct MergeIter<L, R, T> {
    left: L,
    right: R,
    l_next: Option<T>,
    r_next: Option<T>,
    cmp_function: Box<dyn Fn(&T, &T) -> bool>,
}

impl<L, R, T> MergeIter<L, R, T>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
    T: Ord,
{
    /// Creates a new `MergeIter` that returns elements from both supplied iterators in order,
    /// given they are sorted.
    ///
    /// # Examples
    /// ```
    /// use merging_iterator::MergeIter;
    /// let a = vec![0, 2, 4, 6, 8].into_iter();
    /// let b = vec![1, 3, 5, 7, 9].into_iter();
    /// let merger = MergeIter::new(a, b);
    /// assert_eq!(
    ///     vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    ///     merger.collect::<Vec<_>>()
    /// );
    /// ```
    #[inline]
    pub fn new(mut left: L, mut right: R) -> Self {
        Self {
            l_next: left.next(),
            r_next: right.next(),
            left,
            right,
            cmp_function: Box::new(|a, b| a < b),
        }
    }
}

impl<L, R, T> MergeIter<L, R, T>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
{
    /// Creates a new `MergeIter` that uses a custom ordering function.
    ///
    /// # Examples
    /// ```
    /// use merging_iterator::MergeIter;
    /// let a = vec![8, 6, 4, 2, 0].into_iter();
    /// let b = vec![9, 7, 5, 3, 1].into_iter();
    /// let cmp = |a: &u8, b: &u8| b < a;
    /// let merger = MergeIter::with_custom_ordering(a, b, cmp);
    /// assert_eq!(
    ///     vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
    ///     merger.collect::<Vec<_>>()
    /// );
    /// ```
    #[inline]
    pub fn with_custom_ordering<F>(mut left: L, mut right: R, cmp: F) -> Self
    where
        F: 'static + Fn(&T, &T) -> bool,
    {
        Self {
            l_next: left.next(),
            r_next: right.next(),
            left,
            right,
            cmp_function: Box::new(cmp),
        }
    }
}

impl<L, R, T> Iterator for MergeIter<L, R, T>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
    T: Ord,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.l_next.take(), self.r_next.take()) {
            (Some(l), Some(r)) => {
                if (self.cmp_function)(&l, &r) {
                    self.l_next = self.left.next();
                    self.r_next = Some(r);
                    Some(l)
                } else {
                    self.l_next = Some(l);
                    self.r_next = self.right.next();
                    Some(r)
                }
            }
            (Some(l), None) => {
                self.l_next = self.left.next();
                self.r_next = None;
                Some(l)
            }
            (None, Some(r)) => {
                self.r_next = None;
                self.r_next = self.right.next();
                Some(r)
            }
            (None, None) => None,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (l, lo) = self.left.size_hint();
        let (r, ro) = self.right.size_hint();
        (
            l + r,
            match (lo, ro) {
                (Some(lo), Some(ro)) => Some(lo + ro),
                (Some(lo), None) => Some(lo),
                (None, Some(ro)) => Some(ro),
                (None, None) => None,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn test_is_sorted_property(mut a: Vec<i32>, mut b: Vec<i32>) {
            a.sort_unstable();
            b.sort_unstable();
            let merger = MergeIter::new(a.into_iter(), b.into_iter());
            assert!(is_sorted(merger));
        }

        #[test]
        fn test_is_sorted_property_for_multiple_iterators(
            mut a: Vec<i32>,
            mut b: Vec<i32>,
            mut c: Vec<i32>
        ) {
            a.sort_unstable();
            b.sort_unstable();
            c.sort_unstable();
            let merger = MergeIter::new(
                MergeIter::new(a.into_iter(), b.into_iter()),
                c.into_iter()
            );
            assert!(is_sorted(merger));
        }
    }

    #[test]
    fn test_merge_sorted_iterators() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let a = vec![1, 3, 5, 7, 9];
        let b = vec![2, 4, 6, 8];
        let merger = MergeIter::new(a.into_iter(), b.into_iter());
        assert_eq!(expected, merger.collect::<Vec<_>>());

        let a = vec![1, 2, 3, 4, 5];
        let b = vec![6, 7, 8, 9];
        let merger = MergeIter::new(a.into_iter(), b.into_iter());
        assert_eq!(expected, merger.collect::<Vec<_>>());

        let a = vec![3, 5, 6, 8];
        let b = vec![1, 2, 4, 7, 9];
        let merger = MergeIter::new(a.into_iter(), b.into_iter());
        assert_eq!(expected, merger.collect::<Vec<_>>());

        let a = vec![];
        let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let merger = MergeIter::new(a.into_iter(), b.into_iter());
        assert_eq!(expected, merger.collect::<Vec<_>>());

        let a = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let b = vec![];
        let merger = MergeIter::new(a.into_iter(), b.into_iter());
        assert_eq!(expected, merger.collect::<Vec<_>>());
    }

    #[test]
    fn test_multiple_iterators() {
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let a = vec![1, 4, 7];
        let b = vec![2, 5, 8];
        let c = vec![3, 6, 9];
        let merger = MergeIter::new(a.into_iter(), b.into_iter());
        let merger = MergeIter::new(c.into_iter(), merger);
        let merger = merger.collect::<Vec<_>>();
        assert_eq!(expected, merger);
        assert!(is_sorted(merger.iter()));
    }

    fn is_sorted<I, T>(iter: I) -> bool
    where
        I: Iterator<Item = T>,
        T: Ord,
    {
        iter.fold((true, None), |(res, last), next| {
            (res && last.map(|v| v < next).unwrap_or(true), Some(next))
        }).0
    }
}
