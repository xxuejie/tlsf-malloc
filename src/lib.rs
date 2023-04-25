#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

#[link(name = "tlsf", kind = "static")]
extern "C" {
    pub fn tlsf_create_with_pool(mem: *mut c_void, bytes: usize) -> *mut c_void;
    pub fn tlsf_malloc(tlsf: *mut c_void, bytes: usize) -> *mut c_void;
    pub fn tlsf_realloc(tlsf: *mut c_void, ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn tlsf_free(tlsf: *mut c_void, ptr: *mut c_void);
}

static mut __TLSF: *mut c_void = core::ptr::null_mut();

pub fn initialize(buffer: &[u8]) {
    unsafe { __TLSF = tlsf_create_with_pool(buffer.as_ptr() as *mut c_void, buffer.len()) };
}

pub struct TlsfAlloc {}

unsafe impl GlobalAlloc for TlsfAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        tlsf_malloc(__TLSF, layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        tlsf_free(__TLSF, ptr as *mut c_void);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        tlsf_realloc(__TLSF, ptr as *mut c_void, new_size) as *mut u8
    }
}
