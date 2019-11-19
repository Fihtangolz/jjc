use std::ops::Range;
use std::alloc::{Alloc, Global};
use core::ptr::{Unique};

pub struct RingBuffer<T, A: Alloc = Global> { 
    // The internal buffer used for storing elements in the container
    buf: Unique<T>,
    // The internal buffer's end
    end: usize,
    // The virtual beginning of the container
    first: usize,
    // The virtual end of the container
    last: usize,
    // The number of items currently stored in the container
    len: usize,

    a: A,
}

impl<T> RingBuffer<T> {
    pub unsafe fn TEST_NEW() -> RingBuffer<T> {
        let mut t = RingBuffer {
            buf: Unique::empty(),
            end: 0,
            first: 0,
            last: 0,
            len: 0,
            a: Global,
        };

        let res = t.a.alloc_array(21).ok().unwrap();
        t.buf = Unique::new_unchecked(res.as_ptr());
        t.end = (res.as_ptr() as *mut T) as usize + 21;

        return t;
    }
    
    // pub unsafe fn new_inplace(raw: *mut u32) -> RingBuffer<T> {
    //     return ; 
    // }

    // pub fn new() -> RingBuffer<T> {
    //     let mut t = RingBuffer {
    //         buf: Unique::empty(),
    //         end: 0,
    //         first: 0,
    //         last: 0,
    //         len: 0,
    //         a: Global,
    //     };
    // }

    // pub fn with_capacity(capacity: usize) -> RingBuffer<T> {
        
    // }
    
    pub fn push_back(&self, value: T) {
        
        self.len+=1;
    }

    pub fn push_front(&self, value: T) {
        
        self.len+=1;
    }

    pub fn pop_back(&self) -> T {
       
        self.len-=1;
    }

    pub fn pop_front(&self) -> T {
        
        self.len-=1;
    }

    // pub fn range<K, R>(&self, range: R) -> std::ops::Range<> {
    //     //TODO 
    // }
}

// impl<T, A: Alloc> Drop for RingBuffer<T, A> {}

pub trait CapacityInfo {
    // Get the number of elements currently stored in the container
    fn len(&self) -> usize;
    // Get the capacity of the container
    fn capacity(&self) -> usize;
}

impl<T> CapacityInfo for RingBuffer<T> {
    fn len(&self) -> usize {
        return self.len;
    }

    fn capacity(&self) -> usize {
        return self.end - (self.buf.as_ptr() as usize);
    }
}

pub trait CapacityInfoExt {
    fn is_full(&self) -> bool;
    fn is_empty(&self) -> bool;
    fn reserve(&self) -> usize;
}

impl<T: CapacityInfo> CapacityInfoExt for T {
    fn is_full(&self) -> bool {
        return self.capacity() == self.len();
    }

    fn is_empty(&self) -> bool {
        return self.len() == 0;
    }

    fn reserve(&self) -> usize { 
        return self.capacity() - self.len(); 
    }
}