use std::ffi::CStr;
use std::slice;
use std::str;
use libc::{c_char, size_t};

// Take a slice of bytes and try to print it as a UTF-8 string.
fn print_byte_slice_as_utf8(bytes: &[u8]) {
    match str::from_utf8(bytes) {
        Ok(s)    => println!("got {}", s),
        Err(err) => println!("invalid UTF-8 data: {}", err),
    }
}

// Accept an array of bytes and a length and parse it as a UTF-8 string.
#[no_mangle]
pub extern fn utf8_bytes_to_rust(bytes: *const u8, len: size_t) {
    let byte_slice: &[u8] = unsafe { slice::from_raw_parts(bytes, len as usize) };
    print_byte_slice_as_utf8(byte_slice);
}

// Accept a C-style (null-terminated UTF-8 or ASCII data) string.
#[no_mangle]
pub extern fn c_string_to_rust(null_terminated_string: *const c_char) {
    let c_str: &CStr = unsafe { CStr::from_ptr(null_terminated_string) };
    let byte_slice: &[u8] = c_str.to_bytes();
    print_byte_slice_as_utf8(byte_slice);
}

// Helper struct that we'll use to give strings to C.
#[repr(C)]
pub struct RustByteSlice {
    pub bytes: *const u8,
    pub len: size_t,
}

// Give a string back to our caller.
#[no_mangle]
pub extern fn get_string_from_rust() -> RustByteSlice {
    let s = "This is a string from Rust.";
    RustByteSlice{
        bytes: s.as_ptr(),
        len: s.len() as size_t,
    }
}
