use std::alloc::{Alloc, Global};
use std::ops::Index;
use core::slice::SliceIndex;
use std::ptr;
use std::fmt;
use std::mem::MaybeUninit;

pub use crate::help_trait::{CapacityInfo, CapacityManipulator, CapacityInfoExt};

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

pub struct InternalArray <'a, T>{
    con: &'a RingBuffer<T>,
}

impl<'a, T> InternalArray<'a, T> {
     /// Is the internal buffer is linearized into a continuous array
     pub fn is_linearized(&self) -> bool {
        // Elements are not separated by two part (at start or end in internal buffer) 
        return self.con.first < self.con.last 
        // All element placed at the end of internal buffer 
        || self.con.last == 0;
    }

    pub fn linearize(&self) {
        if self.con.is_empty() {
            return;
        }
        if self.is_linearized() {
            return;
        }

    }

    pub fn first_part(&self) -> &[T] {
        unsafe { 
            return std::slice::from_raw_parts(self.con.buf.as_ptr().offset(self.con.first as isize), 0);
        }
    }

    pub fn second_part(&self) -> &[T] {
        unsafe { 
            return std::slice::from_raw_parts(self.con.buf.as_ptr().offset(0), );
        }
    }
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
 
    // Interanal
    fn increment(front_limit: usize, back_limit: usize, offset: &mut usize) {
        *offset += 1;
        if *offset > back_limit {
            *offset = front_limit;
        }
    }

    fn decrement(front_limit: usize, back_limit: usize, offset: &mut usize) {
        if *offset == front_limit {
            *offset = back_limit;
        } else {
            *offset -= 1;   
        }
    }

    // Mapping incoming offset to real offset 
    fn map_offset(&self, offset: usize) -> usize {
        self.first + offset
    }

    pub fn push_back(&mut self, value: T) {
        if self.is_full() {
            if self.is_empty() {
                return;
            }
            /* replace last */
            Self::increment(0, self.cap, &mut self.last)
        } else {
            if self.is_empty() {
                unsafe { ptr::replace(self.buf.as_ptr().offset(self.last as isize), value) };
                return;
            }
            unsafe { ptr::replace(self.buf.as_ptr().offset(self.last as isize), value) };
            Self::increment(0, self.cap, &mut self.last);
        }
    }

    pub fn push_front(&mut self, value: T) {
        if self.is_full() {
            if self.is_empty() {
                return;
            }
            /* replace first */
            Self::decrement(0, self.cap, &mut self.first)
        } else {
            Self::decrement(0, self.cap, &mut self.first);
            unsafe { ptr::replace(self.buf.as_ptr().offset(self.first as isize), value) };
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        };

        Self::decrement(0, self.cap, &mut self.last);

        unsafe { 
            return Some(ptr::read(self.buf.as_ptr().offset(self.last as isize))); 
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        };

        let val = self.first;
        Self::increment(0, self.cap, &mut self.first);
        
        unsafe { 
            return Some(ptr::read(self.buf.as_ptr().offset(val as isize))); 
        }
    }

    pub fn internal_array(&self) -> InternalArray<T> {
        return InternalArray{con: self}
    }

    /// Used to debug internal representation
    ///
    /// # Examples
    /// ```
    ///     let mut rbuff = RingBuffer::with_capacity(6);
    ///     rbuff.push_back(1);
    ///     rbuff.push_back(2);
    ///     rbuff.push_front(6);
    ///     rbuff.push_front(5);
    ///     
    ///     rbuff.debug_internals();
    /// ```
    /// 
    /// This outputs:
    /// 
    /// ```text
    ///      [1, 2, uninit, uninit, 5, 6]
    ///          ^                  ^ 
    ///          l                  f 
    /// ```
    pub fn debug_internals(&self) {
        // use std::string::String;

        // struct SublinePlacement {
        //     len: usize,
        //     offset: usize,
        // }

        // let mut sb_first = SublinePlacement{len:0, offset:0};
        // let mut sb_last = SublinePlacement{len:0, offset:0};
        // let mut ret = std::string::String::new();

        // write!(f, "[");
        // for indx in 1..=self.cap {
            
        //     unsafe {
        //         let el = self.buf.as_ptr().offset(indx as isize);
        //         // let st = "XCD";
        //         fmt::Debug::fmt(&*el, f); 
        //     }
        //     if indx != self.cap { write!(f, ", "); }

        //     if self.first == indx-1 {
        //         // sb_first.len = st.len();
        //         sb_last.offset = ret.len();
        //     }

        //     if self.last == indx {
        //         // sb_last.len = st.len();
        //         sb_last.offset = ret.len();
        //     }

        //     // ret.push_str(st);
        // }
        // // let mut koko = String::from(" ").repeat(tet.offset+1);
        // // let mut ui = koko.clone();
        // // ui.push_str(&String::from("^").repeat(tet.len));
        // // let mut bu = koko.clone();
        // // bu.push('l');
        // return write!(f, "]");
    }

    //Clearing 

    /// Removes all stored elements
    pub fn clear(&mut self) {
        let mut cur = self.first;
        unsafe { std::ptr::drop_in_place(self.buf.as_ptr().offset(cur as isize)) };
        Self::increment(0, self.cap, &mut cur);
    }

    /// Removes item at position index  
    pub fn remove(&mut self, index: usize) {

    }

    // Remove items at range 
    

    //К примеру: применить придикат и удалить и вот тут вопрос нам еше нужны удалить или хватит
}

impl<T> Index<usize> for RingBuffer<T> {
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len() {
            return &None;
        }
        
        let real_offset = self.map_offset(index);
        unsafe { 
            return &None;
            // return &Some(ptr::read(self.buf.as_ptr().offset(real_offset as isize))) 
        }; 
    }
}

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

            // if self.is_linearized() {
            //     std::ptr::copy_nonoverlapping(self.buf.as_ptr(), copy.buf.as_ptr(), self.capacity()); 
            // }
            // std::ptr::copy_nonoverlapping(self.buf.as_ptr(), copy.buf.as_ptr(), self.capacity()); 
            // std::ptr::copy_nonoverlapping(self.buf.as_ptr(), copy.buf.as_ptr(), self.capacity()); 

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

impl<T> CapacityManipulator<T> for RingBuffer<T> {
    fn resize(&mut self, new_len: usize, value: T) {
        
    }

    fn reserve(&mut self, additional: usize) {
        
    }

    fn shrink_to_fit(&mut self) {

    }
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

impl<T> CapacityInfoExt for RingBuffer<T> {
    fn is_full(&self) -> bool {
        return self.first - 1 == self.last;
    }

    fn is_empty(&self) -> bool {
        return self.first == self.last;
    }

    fn reserve_size(&self) -> usize {
        return self.capacity() - self.len();
    }
}