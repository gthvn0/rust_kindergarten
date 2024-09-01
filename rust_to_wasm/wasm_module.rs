#![no_main]

extern "C" {
    fn ext_log(msg: *const u8, len: i32);
    fn ext_draw_rectangle(x: i32, y: i32, w: i32, h: i32, color: u32);
}

#[no_mangle]
pub fn update() {
    // TODO
}

#[no_mangle]
pub fn render() {
    let blue = 0x00_00_FF_FF_u32;
    unsafe {
        ext_draw_rectangle(0, 0, 30, 30, blue);
    }
}

#[no_mangle]
pub fn init(width: i32, height: i32) {
    let msg = format!("width: {}, height: {}", width, height);

    unsafe {
        ext_log(msg.as_ptr(), msg.len() as i32);
    }
}
