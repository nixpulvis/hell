/// A fail-fast macro for evaluating boolean expressions, returns `false` if any of the given
/// expressions evaluate to `false`.
///
/// # Examples
///
/// ```
/// fn single_example(n: u64) -> bool {
///     try_bool!(n % 2 == 0);
///     true
/// }
///
/// fn multiple_example(n: u64) -> bool {
///     try_bool((n % 3 == 0), (n % 5 == 0));
///     true
/// }
///
/// assert!(single_example(2));
/// assert!(!single_example(7));
///
/// assert!(multiple_example(15));
/// assert!(!multiple_example(7));
/// ```
macro_rules! try_bool {
    ( $cond:expr ) => ( if !$cond { return false; } );
    ( $( $cond:expr ),+ ) => ( $( try_bool!($cond) )+ );
}

/// Get an around from iterator from a finite collection.
pub trait AroundFrom<'a, T> {
    fn around_from(self, start: usize) -> AroundFromIter<'a, T>;
}

/// Get an around from mut iterator from a finite collection.
pub trait AroundFromMut<'a, T> {
    fn around_from_mut(self, start: usize) -> AroundFromMutIter<'a, T>;
}

/// An iterator which starts at a given index and wraps around the
/// end yielding references to every element.
pub struct AroundFromIter<'a, T> where T: 'a {
    /// The index that will be visited on the next iteration.
    current: usize,
    /// The number of indices that have already been visited.
    count: usize,
    /// The slice to iterate over.
    slice: &'a [T]
}

impl<'a, T> Iterator for AroundFromIter<'a, T> {
    type Item = (usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.slice.len() {
            let element = &self.slice[self.current];
            let index = self.current;
            self.current = (self.current + 1) % self.slice.len();
            self.count += 1;
            Some((index, element))
        } else {
            None
        }
    }
}

/// An iterator which starts at a given index and wraps around the
/// end yielding mutable references to every element.
pub struct AroundFromMutIter<'a, T> where T: 'a {
    /// The index that will be visited on the next iteration.
    current: usize,
    /// The number of indices that have already been visited.
    count: usize,
    /// The slice to iterate over.
    slice: &'a mut [T]
}

impl<'a, T> Iterator for AroundFromMutIter<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.slice.len() {
            let element: *mut T = &mut self.slice[self.current];
            let index = self.current;
            self.current = (self.current + 1) % self.slice.len();
            self.count += 1;
            Some((index, unsafe { &mut *element }))
        } else {
            None
        }
    }
}

impl<'a, T> AroundFrom<'a, T> for &'a [T] {
    fn around_from(self, start: usize) -> AroundFromIter<'a, T> {
        AroundFromIter {
            current: start,
            count: 0,
            slice: self
        }
    }
}

impl<'a, T> AroundFromMut<'a, T> for &'a mut [T] {
    fn around_from_mut(self, start: usize) -> AroundFromMutIter<'a, T> {
        AroundFromMutIter {
            current: start,
            count: 0,
            slice: self
        }
    }
}

/// Remove the element at the first index of an indexed colection.
pub trait Dequeue<'a, T> {
    fn dequeue(&mut self) -> Option<T>;
}

impl<'a, T> Dequeue<'a, T> for Vec<T> {
    fn dequeue(&mut self) -> Option<T> {
        if self.len() < 1 {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use ext::*;

    #[test]
    fn try_bool_single() {
        fn is_even(n: u64) -> bool {
            try_bool!(n % 2 == 0);
            true
        }

        assert!(is_even(4));
        assert!(!is_even(7));
    }

    #[test]
    fn try_bool_multiple() {
        fn is_fizzbuzz(n: u64) -> bool {
            try_bool!((n % 3 == 0), (n % 5 == 0));
            true
        }

        assert!(is_fizzbuzz(15));
        assert!(!is_fizzbuzz(7));
    }

    #[test]
    fn around_from_iter() {
        let vector = vec![0, 1, 2, 3, 4];
        let expected = vec![2, 3, 4, 0, 1];
        let actual = vector.around_from(2).map(|(_, item)| *item).collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn around_from_iter_index() {
        let vector = vec![0, 1, 2, 3, 4];
        let expected = vec![2, 3, 4, 0, 1];
        let actual = vector.around_from(2).map(|(i, _)| i).collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }

    #[test]
    fn around_from_iter_mut() {
        let mut vector = vec![0, 1, 2];
        let mut iterator = vector.around_from_mut(1);
        assert_eq!(Some((1, &mut 1)), iterator.next());
        assert_eq!(Some((2, &mut 2)), iterator.next());
        assert_eq!(Some((0, &mut 0)), iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn dequeue() {
        let mut vector = vec![0, 1, 2];
        assert_eq!(Some(0), vector.dequeue());
        assert_eq!(Some(1), vector.dequeue());
        assert_eq!(Some(2), vector.dequeue());
        assert_eq!(None, vector.dequeue());
    }
}
