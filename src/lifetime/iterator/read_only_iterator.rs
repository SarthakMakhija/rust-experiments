/*
 ReadonlyIterator has a lifetime that is tied to the lifetime the slice.
 ReadonlyIterator does not outlive the the slice.
*/
pub(crate) struct ReadonlyIterator<'iterator, T> {
    slice: &'iterator [T],
}

impl<'iterator, T> ReadonlyIterator<'iterator, T> {
    pub(crate) fn new(slice: &'iterator [T]) -> Self {
        return ReadonlyIterator { slice };
    }
}

impl<'iterator, T> Iterator for ReadonlyIterator<'iterator, T> {
    /*
        Iterator will return the Item that is a reference to T which has the lifetime of ReadonlyIterator struct.
    */
    type Item = &'iterator T;

    /**
        &mut self has an anonymous lifetime. This means `self` is borrowed mutably for a lifetime that is as long as that of the next method.
        If `self` is is borrowed mutably for a lifetime that is as long as that of the next method, then how does this method return an Option
        with a reference to T with the lifetime that is as long as the ReadonlyIterator struct.

        It looks like rust is extending the lifetime of the readonly reference because slice is `&'iterator [T]`.
    */
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }
        let element = self.slice.get(0);
        self.slice = &self.slice[1..];

        return element;
    }
}

#[cfg(test)]
mod tests {
    use super::ReadonlyIterator;

    #[test]
    fn iterate_over_a_slice_of_n_elements() {
        let elements = vec![1, 2, 3, 4, 5];
        let iterator = ReadonlyIterator::new(&elements[..]);

        for (index, element) in iterator.enumerate() {
            let expected = elements[index];
            assert_eq!(expected, *element);
        }
    }

    #[test]
    fn attempt_to_iterate_over_an_empty_slice() {
        let elements: Vec<i32> = vec![];
        let mut iterator = ReadonlyIterator::new(&elements[..]);

        assert_eq!(None, iterator.next());
    }
}
