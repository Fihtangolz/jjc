use std::iter::Iterator;

pub struct BipBuffer<T> {
    buf: *mut T,
    cap: usize,

    // Offset to start A region 
    a_start: usize,
    // Offset to end A region 
    a_end: usize,
    // Offset to start B region 
    b_start: usize,
    // Offset to end B region 
    b_end: usize,
    // Offset to start reserv  
    reserv_start: usize,
    // Offset to end reserv  
    reserv_end: usize,
}

impl<T> BipBuffer<T> {
    pub fn new() -> Self {
        return Self {
            buf: std::ptr::null_mut(),
            cap: 0,
            a_start: 0,
            a_end: 0,
            b_start: 0, 
            b_end: 0,
            reserv_start: 0,
            reserv_end: 0,
        }
    }

    pub fn clear(&mut self) {
        for idx in (self.a_start..=self.a_end).chain(self.b_start..=self.b_end) {
            unsafe { std::ptr::drop_in_place(self.buf.offset(idx as isize)); }
        }

        self.a_start = 0;
        self.a_end = 0;
        self.b_start = 0;
        self.b_end = 0;
    }

    // pub fn commit() {
        
    // }

    // pub fn reserve() {

    // }
}

pub struct IterStateHolder<'a, T> {
    buf: &'a BipBuffer<T>,
    cursor: usize,
}

impl<'a, T> Iterator for IterStateHolder<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        
        return None;
    }
}

impl<'a, T> IntoIterator for  &'a BipBuffer<T> {
    type Item = &'a T;
    type IntoIter = IterStateHolder<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return IterStateHolder{
            buf: self,
            cursor: ,
        }
    }
}




