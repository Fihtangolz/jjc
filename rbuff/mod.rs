use std::alloc::{Alloc, Global};
use std::mem;
use std::ptr;
use std::fmt;

pub struct RingBuffer<T, A: Alloc = Global> {
    /// The internal buffer used for storing elements in the container
    buf: ptr::Unique<T>,
    /// The offset to buffer's end
    cap: usize,
    /// The offset to virtual beginning of the container
    first: usize,
    /// The offset to virtual end of the container
    last: usize,

    a: A,
}

impl<T> RingBuffer<T> {
    // Construction

    /// Create an empty container with zero capacity
    pub fn new() -> RingBuffer<T> {
        return RingBuffer {
            buf: ptr::Unique::empty(),
            cap: 0,
            first: 0,
            last: 0,
            a: Global,
        };
    }

    /// Create an empty container with the specified capacity
    pub fn with_capacity(capacity: usize) -> RingBuffer<T> {
        let mut t = RingBuffer {
            buf: ptr::Unique::empty(),
            cap: 0,
            first: 0,
            last: 0,
            a: Global,
        };

        let res: ptr::NonNull<T> = t.a.alloc_array(capacity).ok().unwrap();
        unsafe {
            t.buf = ptr::Unique::new_unchecked(res.as_ptr());
            t.cap = capacity;
        }

        return t;
    }

    fn increment(limit: usize, offset: &mut usize) {
        *offset += 1;
        if *offset > limit {
            *offset = 0;
        }
    }

    fn decrement(limit: usize, offset: &mut usize) {
        if *offset == 0 {
            *offset = limit;
        }
        *offset -= 1;
    }

    pub fn push_back(&mut self, value: T) {
        if self.is_full() {
            if self.is_empty() {
                return;
            }
            /* replace last */
            Self::increment(self.cap, &mut self.last)
        } else {
            unsafe { ptr::replace(self.buf.as_ptr().offset(self.last as isize), value) };
            Self::increment(self.cap, &mut self.last);
        }
    }

    pub fn push_front(&mut self, value: T) {
        if self.is_full() {
            if self.is_empty() {
                return;
            }
            /* replace first */
            Self::decrement(self.cap, &mut self.first)
        } else {
            Self::decrement(self.cap, &mut self.first);
            unsafe { ptr::replace(self.buf.as_ptr().offset(self.first as isize), value) };
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        };

        Self::decrement(self.cap, &mut self.last);

        unsafe { 
            return Some(ptr::read(self.buf.as_ptr().offset(self.last as isize))); 
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        };

        let val = self.first;
        Self::increment(self.cap, &mut self.first);
        
        unsafe { 
            return Some(ptr::read(self.buf.as_ptr().offset(val as isize))); 
        }
    }

    /// Is the internal buffer is linearized into a continuous array
    pub fn is_linearized(&self) -> bool {
        // Element are not separated by two part (at start or end in internal buffer) 
        return self.first < self.last 
        // All element placed at the end of internal buffer 
        || self.last == 0;
    }

    pub fn linearize(&self) {
        if self.is_empty() {
            return;
        }
        if self.is_linearized() {
            return;
        }
    }
}

// impl<T> Drop for RingBuffer<T> {
//     fn drop(&mut self) {
//         println!("Dropping!");
//     }
// }

// impl<T> fmt::Debug for RingBuffer<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "RingBuffer = []", )
//     }
// }

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

impl<T> CapacityInfo for RingBuffer<T> {
    fn len(&self) -> usize {
        if self.cap == 1 {
            return 1
        }

        if self.first <= self.last {
            return self.last - self.first;
        }

        return self.cap - (self.first - self.last) 
    }

    fn capacity(&self) -> usize {
        return self.cap;
    }
}

pub trait CapacityInfoExt {
    /// Returns true if the container can't hold more elements
    fn is_full(&self) -> bool;
    /// Returns true if the container contains no elements
    fn is_empty(&self) -> bool;
    /// Returns the number
    fn reserve_size(&self) -> usize;
}

impl<T: CapacityInfo> CapacityInfoExt for T {
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
