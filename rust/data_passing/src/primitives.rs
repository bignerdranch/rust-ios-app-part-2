use libc::size_t;

#[no_mangle]
pub extern fn return_int32() -> i32 {
    10
}

#[no_mangle]
pub extern fn triple_a_uint16(x: u16) -> u16 {
    x * 3
}

#[no_mangle]
pub extern fn return_float() -> f32 {
    10.0
}

#[no_mangle]
pub extern fn average_two_doubles(x: f64, y: f64) -> f64 {
    (x + y) / 2.0
}

#[no_mangle]
pub extern fn sum_sizes(x: size_t, y: size_t) -> size_t {
    let x_usize = x as usize;
    let y_usize = y as usize;
    (x_usize + y_usize) as size_t
}
