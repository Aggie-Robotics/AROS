use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;

use ansi_rgb::{Background, Foreground, red, white};
use cty::c_void;

use crate::raw::pros::api::printf;
use crate::raw::str_to_char_ptr;

#[global_allocator]
static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator{};

struct GlobalAllocator{}
unsafe impl GlobalAlloc for GlobalAllocator{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void)
    }
}
#[alloc_error_handler]
unsafe fn alloc_error_handler(layout: Layout) -> !{
    printf(str_to_char_ptr(format!("{}", format!("Alloc Error: layout: {:?}", layout).bg(red()).fg(white())).as_str()).as_ptr());
    loop{}
}

#[panic_handler]
unsafe fn panic_handler(panic_info: &PanicInfo) -> !{
    printf(str_to_char_ptr(format!("{}", format!("Panic! Info: {:?}", panic_info).bg(red()).fg(white())).as_str()).as_ptr());
    loop {}
}

extern "C"{
    fn malloc(length: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}


