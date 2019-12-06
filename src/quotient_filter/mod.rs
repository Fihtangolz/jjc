use core::hash::Hash;

bitflags! {
    struct Flags: u8 {
        const OCCUPIED = 0b00000001;
        const CONTINUATION = 0b00000010;
        const SHIFTED = 0b00000100;
    }
}

pub struct QuotientFilter<T> {
    slots: Vec<Slot<T>>,

    element_bits: u8,
    qbits: u8,
    rbits: u8,
    rmask: u64,
    elem_mask: u64,
    max_size: usize,
    // ...
}

impl<T> QuotientFilter<T>
where
    T: Hash,
{
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            element_bits: 0,
            qbits: 0,
            rbits: 0,
            rmask: 0,
            elem_mask: 0,
            max_size: 0,
        }
    }

    pub fn insert(&mut self, element: T) -> bool {
        if self.slots.len() >= self.max_size {
            return false;
        }


        false
    }

    pub fn may_contain(&self, element: &T) -> bool {
        false
    }

    pub fn remove(&mut self, element: &T) -> bool {
        false
    }

    pub fn clear(&mut self) {
        self.slots.clear();
    }

    pub fn merge(&mut self, another: &mut Self) -> Self {
        let mut result = Self::with_capacity(self.slots.len() + another.slots.len());
        result.slots.append(&mut self.slots);
        result.slots.append(&mut another.slots);
        result
    }
}

struct Slot<T> {
    bits: Flags,
    value: T,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn size_of_flags_1() {
        debug_assert_eq!(mem::size_of::<Flags>(), 1);
    }
}