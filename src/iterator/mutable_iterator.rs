pub(crate) struct MutableIterator<'iterator, T> {
    slice: &'iterator mut [T],
}

impl<'iterator, T> MutableIterator<'iterator, T> {
    pub(crate) fn new(slice: &'iterator mut [T]) -> Self {
        return MutableIterator { slice };
    }
}

impl<'iterator, T> Iterator for MutableIterator<'iterator, T> {
    type Item = &'iterator mut T;

    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let slice = &mut self.slice;
        let original_slice = std::mem::replace(slice, &mut []);

        let (head, tail) = original_slice.split_first_mut()?;
        self.slice = tail;

        return Some(head);
    }
}

#[cfg(test)]
mod tests {
    use crate::iterator::mutable_iterator::MutableIterator;

    #[test]
    fn iterate_over_a_slice_of_n_elements() {
        let mut elements = vec![1, 2, 3, 4, 5];
        let iterator = MutableIterator::new(&mut elements[..]);

        for (_index, element) in iterator.enumerate() {
            *element = *element + 1;
        }

        assert_eq!(vec![2, 3, 4, 5, 6], elements);
    }

    #[test]
    fn attempt_to_iterate_over_an_empty_slice() {
        let mut elements: Vec<i32> = vec![];
        let mut iterator = MutableIterator::new(&mut elements[..]);

        assert_eq!(None, iterator.next());
    }
}
