pub(crate) struct ReadonlyIterator<'iterator, T> {
    slice: &'iterator [T],
}

impl<'iterator, T> ReadonlyIterator<'iterator, T> {
    pub(crate) fn new(slice: &'iterator [T]) -> Self {
        return ReadonlyIterator { slice };
    }
}

impl<'iterator, T> Iterator for ReadonlyIterator<'iterator, T> {
    type Item = &'iterator T;

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
