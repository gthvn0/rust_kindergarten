#![no_main]

#[no_mangle]
pub fn sum3(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}
