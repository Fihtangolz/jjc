use std::alloc::{Alloc, Global};
use std::ops::Index;
use core::slice::SliceIndex;
use std::ptr;
use std::fmt;
use std::mem::MaybeUninit;

pub struct RingBuffer<T> {
    /// The internal buffer used for storing elements in the container
    buf: ptr::Unique<T>,
    /// The offset to buffer's end
    cap: usize,
    /// The offset to virtual beginning of the container
    first: usize,
    /// The offset to virtual end of the container
    last: usize,

    a: Global,
}

impl<T> RingBuffer<T> {
    // Construction

    /// Create an empty invariant container with zero capacity
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

    pub fn new_inplace(raw: *mut u32) -> RingBuffer<T> {
        unimplemented!();
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
        // Elements are not separated by two part (at start or end in internal buffer) 
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

impl<T: fmt::Debug> fmt::Debug for RingBuffer<T> {
    //TODO: expected implementation
    //
    //      [1, 2, 3, 4, 5, 6]
    //       ^           ^ 
    //       f           l 
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::string::String;

        struct SublinePlacement {
            len: usize,
            offset: usize,
        }

        let mut sb_first = SublinePlacement{len:0, offset:0};
        let mut sb_last = SublinePlacement{len:0, offset:0};
        let mut ret = std::string::String::new();

        write!(f, "[");
        for indx in 1..=self.cap {
            
            unsafe {
                let el = self.buf.as_ptr().offset(indx as isize);
                // let st = "XCD";
                fmt::Debug::fmt(&*el, f); 
            }
            if indx != self.cap { write!(f, ", "); }

            if self.first == indx-1 {
                // sb_first.len = st.len();
                sb_last.offset = ret.len();
            }

            if self.last == indx {
                // sb_last.len = st.len();
                sb_last.offset = ret.len();
            }

            // ret.push_str(st);
        }
        // let mut koko = String::from(" ").repeat(tet.offset+1);
        // let mut ui = koko.clone();
        // ui.push_str(&String::from("^").repeat(tet.len));
        // let mut bu = koko.clone();
        // bu.push('l');
        return write!(f, "]");
    }
}

// impl<T,I> Index<I> for RingBuffer<T> {
//     type Output;

//     fn index(&self, index: I) -> &Self::Output {
//        unimplemented!();
//     }
// }

impl<T> Clone for RingBuffer<T> {
    fn clone(&self) -> Self {
        unsafe {
            let mut copy: RingBuffer<T> = MaybeUninit::uninit().read();
       
            copy.cap = self.cap;
            copy.first = self.first;
            copy.last = self.last;  
            copy.a = self.a;
            
            let res: ptr::NonNull<T> = copy.a.alloc_array(copy.cap).ok().unwrap();
            copy.buf = ptr::Unique::new_unchecked(res.as_ptr());

            std::ptr::copy_nonoverlapping(self.buf.as_ptr(), copy.buf.as_ptr(), self.capacity()); 
            //TODO: may be optimized

            return copy;
        }
    }

    fn clone_from(&mut self, source: &Self) {
        if self.capacity() >= source.capacity() {
            //TODO: just copy internals, also all object need to be correct droped
        }
        //TODO: ...
    }
} 

pub struct IterStateHolder<'a, T> {
    ptr: *const T,
    cur: usize,
    last: usize,
    phantom: &'a std::marker::PhantomData<T>
}

impl<'a, T> Iterator for IterStateHolder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.last { //TODO: incorect implementation
            unsafe { 
                let el = self.ptr.offset(self.cur as isize);
                self.cur += 1;
                return Some(&*el);
            } 
        }

        return None;
    }
}

impl<'a, T> IntoIterator for  &'a RingBuffer<T> {
    type Item = &'a T;
    type IntoIter = IterStateHolder<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return IterStateHolder{
            ptr: self.buf.as_ptr(),
            cur: self.first,
            last: self.last,
            phantom: &std::marker::PhantomData
        }
    }
}

impl<T> Drop for RingBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            let ptr = ptr::NonNull::new_unchecked(self.buf.as_ptr());
            self.a.dealloc_array(ptr, self.cap);
        };
    }
}

pub trait CapacityManipulator<T> {
    fn resize(&mut self, new_len: usize, value: T);
    fn reserve(&mut self, additional: usize);
    fn shrink_to_fit(&mut self);
}

impl<T> CapacityManipulator<T> for RingBuffer<T> {
    fn resize(&mut self, new_len: usize, value: T) {
        
    }

    fn reserve(&mut self, additional: usize) {
        
    }

    fn shrink_to_fit(&mut self) {

    }
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
