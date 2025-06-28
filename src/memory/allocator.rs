
struct Allocator{
    

}
impl Allocator{
    pub fn new()->Self{
        Allocator{

        }
    }
}
unsafe impl GlobalAlloc for Allocator{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unimplemented!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unimplemented!()
    }
}