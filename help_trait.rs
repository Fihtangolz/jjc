pub trait CapacityManipulator<T> {
    fn resize(&mut self, new_len: usize, value: T);
    fn reserve(&mut self, additional: usize);
    fn shrink_to_fit(&mut self);
}

pub trait CapacityInfo {
    /// Returns the number of elements currently stored in the container
    fn len(&self) -> usize;
    /// Returns the number of elements that can be held in container
    fn capacity(&self) -> usize;
}

pub trait CapacityInfoExt {
    /// Returns true if the container can't hold more elements
    fn is_full(&self) -> bool;
    /// Returns true if the container contains no elements
    fn is_empty(&self) -> bool;
    /// Returns the number
    fn reserve_size(&self) -> usize;
}

default impl<T: CapacityInfo> CapacityInfoExt for T {
    fn is_full(&self) -> bool {
        return self.capacity() == self.len();
    }

    fn is_empty(&self) -> bool {
        return self.len() == 0;
    }

    fn reserve_size(&self) -> usize {
        return self.capacity() - self.len();
    }
}