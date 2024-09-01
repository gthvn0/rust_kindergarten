#![no_main]

extern "C" {
    fn ext_log(msg: *const u8, len: i32);
    fn ext_draw_rectangle(x: u32, y: u32, w: u32, h: u32, color: u32);
}

static WHITE: u32 = 0xFF_FF_FF_FF;
static BLACK: u32 = 0x00_00_00_FF;

struct Game {
    width: u32,
    height: u32,
    rec_x: u32,
    rec_y: u32,
    rec_width: u32,
    rec_height: u32,
}

static mut GAME: Game = Game {
    height: 0,
    width: 0,
    rec_x: 1,
    rec_y: 1,
    rec_width: 30,
    rec_height: 30,
};

fn clear_screen() {
    unsafe {
        ext_draw_rectangle(
            0,
            0,
            GAME.width,
            GAME.height,
            BLACK,
        );
    }
}

#[no_mangle]
pub fn update() {
    unsafe {
        GAME.rec_x += 1;
        GAME.rec_y += 1;
    }
}

#[no_mangle]
pub fn render() {
    clear_screen();
    unsafe {
        ext_draw_rectangle(
            GAME.rec_x,
            GAME.rec_y,
            GAME.rec_width,
            GAME.rec_height,
            WHITE,
        );
    }
}

#[no_mangle]
pub fn init(width: u32, height: u32) {
    let msg = format!("width: {}, height: {}", width, height);

    unsafe {
        GAME.height = height;
        GAME.width = width;
        ext_log(msg.as_ptr(), msg.len() as i32);
    }
}
