/*
 MutableIterator has a lifetime that is tied to the lifetime the slice.
 MutableIterator does not outlive the the slice.
*/
pub(crate) struct MutableIterator<'iterator, T> {
    slice: &'iterator mut [T],
}

impl<'iterator, T> MutableIterator<'iterator, T> {
    pub(crate) fn new(slice: &'iterator mut [T]) -> Self {
        return MutableIterator { slice };
    }
}

impl<'iterator, T> Iterator for MutableIterator<'iterator, T> {
    /*
        Iterator will return the Item that is a reference to T which has the lifetime of MutableIterator struct.
    */
    type Item = &'iterator mut T;

    /**
        &mut self has a lifetime indicated by `next`. This means `self` is borrowed mutably for a lifetime that is as long as that of the next method.
        If `self` is is borrowed mutably for a lifetime that is as long as that of the next method, then how does this method return an Option
        with a reference to T with the lifetime that is as long as the MutableIterator struct.

        Rust is not extending the lifetime of the mutable reference because slice is `&'iterator mut [T]`.
        In order to deal with this, the code performs `std::mem::replace(slice, &mut []);` and gets the original slice that has the `iterator` lifetime.
        We work with `original_slice`, get the item and then replace the slice.
    */
    fn next<'next>(&'next mut self) -> Option<Self::Item> {
        let original_slice: &'iterator mut [T] = std::mem::replace(&mut self.slice, &mut []);

        let (head, tail) = original_slice.split_first_mut()?;
        self.slice = tail;

        return Some(head);
    }
}

#[cfg(test)]
mod tests {
    use crate::lifetime::iterator::mutable_iterator::MutableIterator;

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
