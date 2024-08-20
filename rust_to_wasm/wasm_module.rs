#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern "C" {
    fn loggme(msg: *const u8, len: i32);
}

#[no_mangle]
pub fn a_sum(a: i32, b: i32) -> i32 {
    let msg = "The answer is";
    let msg_len = msg.len() as i32;

    unsafe {
        loggme(msg.as_ptr(), msg_len);
    }
    a + b
}
