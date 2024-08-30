use std::ffi::c_int;

extern "C" {
    fn GetNumber() -> c_int;
}

fn main() {
    let answer = unsafe { GetNumber() };
    println!("Answer is {}", answer);
}
