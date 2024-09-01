#![no_main]

extern "C" {
    fn ext_log(msg: *const u8, len: i32);
    fn ext_draw_rectangle(x: u32, y: u32, w: u32, h: u32, color: u32);
}

static WHITE: u32 = 0xFF_FF_FF_FF;
static BLACK: u32 = 0x00_00_00_FF;

struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    speed_x: i32,
    speed_y: i32,
}

struct Game {
    width: u32,
    height: u32,
    rect: Rectangle,
}

static mut GAME: Game = Game {
    height: 0,
    width: 0,
    rect: Rectangle {
        x: 1,
        y: 1,
        width: 30,
        height: 30,
        speed_x: 1,
        speed_y: 1,
    },
};

fn set_game(game: &mut Game) {
    unsafe {
        GAME.height = game.height;
        GAME.width = game.width;
        GAME.rect.x = game.rect.x;
        GAME.rect.y = game.rect.y;
        GAME.rect.width = game.rect.width;
        GAME.rect.height = game.rect.height;
        GAME.rect.speed_x = game.rect.speed_x;
        GAME.rect.speed_y = game.rect.speed_y;
    }
}

fn clear_screen() {
    unsafe {
        ext_draw_rectangle(0, 0, GAME.width, GAME.height, BLACK);
    }
}

#[no_mangle]
pub fn key(k: u32) {
    match k {
        0 => unsafe { GAME.rect.speed_y = -1 }, // Up
        1 => unsafe { GAME.rect.speed_x = 1 },  // Right
        2 => unsafe { GAME.rect.speed_y = 1 },  // Down
        3 => unsafe { GAME.rect.speed_x = -1 }, // Left
        _ => {}                                 // Nothing to do
    }
}

#[no_mangle]
pub fn update() {
    // Get the current state of the game
    let mut game: Game = unsafe {
        Game {
            height: GAME.height,
            width: GAME.width,
            rect: Rectangle {
                x: GAME.rect.x,
                y: GAME.rect.y,
                width: GAME.rect.width,
                height: GAME.rect.height,
                speed_x: GAME.rect.speed_x,
                speed_y: GAME.rect.speed_y,
            },
        }
    };

    let new_x = game.rect.x as i32 + game.rect.speed_x;
    if 0 <= new_x && new_x <= (game.width as i32 - game.rect.width as i32) {
        game.rect.x = new_x as u32;
    }

    let new_y = game.rect.y as i32 + game.rect.speed_y;
    if 0 <= new_y && new_y <= (game.height as i32 - game.rect.height as i32) {
        game.rect.y = new_y as u32;
    }

    set_game(&mut game);
}

#[no_mangle]
pub fn render() {
    clear_screen();
    unsafe {
        ext_draw_rectangle(
            GAME.rect.x,
            GAME.rect.y,
            GAME.rect.width,
            GAME.rect.height,
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
