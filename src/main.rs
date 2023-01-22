use std::ffi::{c_int, c_uchar};
use std::ptr;

extern "C" {
    fn parse_data(ignore: *const c_int, out: *mut c_uchar, input: *const c_uchar, inputlen: c_int) -> c_int;
}

fn main() {
    let mut dummy = String::new();
    dummy.push_str("dummy");

    unsafe {
        parse_data(ptr::null(), ptr::null_mut(), ptr::null(), 33);
    }
}
