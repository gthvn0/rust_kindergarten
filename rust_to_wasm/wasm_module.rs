#![no_main]

extern "C" {
    fn ext_log(msg: *const u8, len: i32);
    fn ext_draw_rectangle(x: u32, y: u32, w: u32, h: u32, color: u32);
}

static WHITE: u32 = 0xFF_FF_FF_FF;
static BLACK: u32 = 0x00_00_00_FF;

pub struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    speed_x: i32,
    speed_y: i32,
}

pub struct Game {
    width: u32,
    height: u32,
    rect: Rectangle,
}

#[no_mangle]
pub fn key(game: &mut Game, k: u32) {
    match k {
        0 => game.rect.speed_y = -1, // Up
        1 => game.rect.speed_x = 1,  // Right
        2 => game.rect.speed_y = 1,  // Down
        3 => game.rect.speed_x = -1, // Left
        _ => {}                      // Nothing to do
    }
}

#[no_mangle]
pub fn update(game: &mut Game) {
    let new_x = game.rect.x as i32 + game.rect.speed_x;
    if 0 <= new_x && new_x <= (game.width as i32 - game.rect.width as i32) {
        game.rect.x = new_x as u32;
    }

    let new_y = game.rect.y as i32 + game.rect.speed_y;
    if 0 <= new_y && new_y <= (game.height as i32 - game.rect.height as i32) {
        game.rect.y = new_y as u32;
    }
}

#[no_mangle]
pub fn render(game: &mut Game) {
    unsafe {
        // Clear screen
        ext_draw_rectangle(0, 0, game.width, game.height, BLACK);
        // Draw the rectangle
        ext_draw_rectangle(
            game.rect.x,
            game.rect.y,
            game.rect.width,
            game.rect.height,
            WHITE,
        );
    }
}

#[no_mangle]
pub fn init(width: u32, height: u32) -> *mut Game {
    let my_game: Box<Game> = Box::new(Game {
        height,
        width,
        rect: Rectangle {
            x: 1,
            y: 1,
            width: 30,
            height: 30,
            speed_x: 1,
            speed_y: 1,
        },
    });

    unsafe {
        let msg = format!("Game initialized: width: {}, height: {}", width, height);
        ext_log(msg.as_ptr(), msg.len() as i32);
    }

    Box::into_raw(my_game)
}
