#![allow(unsafe_op_in_unsafe_fn)]
use libc;


unsafe fn unsafe_print() {
    libc::printf(
        "calling C's printf() within unsafe_function()\n\0".as_ptr()
        as *const i8,
    );
}

fn main() {
    unsafe {
        unsafe_print();
    }
}