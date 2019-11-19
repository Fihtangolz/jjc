trait BasicsAllocator {
    fn allocate(num: usize) -> *mut u32;
    fn deallocate(ptr: *mut u32, num: usize);
} //TODO: comoparation 

