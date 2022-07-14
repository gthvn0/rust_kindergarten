use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let mut game: GameInfo = GameInfo::default();
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    game.width = parse_input!(inputs[0], i32); // width of the building.
    game.height = parse_input!(inputs[1], i32); // height of the building.
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    game.turn = parse_input!(input_line, i32); // maximum number of turns before game over.

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    game.position.x = parse_input!(inputs[0], i32);
    game.position.y = parse_input!(inputs[1], i32);

    // game loop
    println!("GameInfo {:?}", game);
    
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

        game.update_position(&bomb_dir);

        // the location of the next window Batman should jump to.
        println!("{} {}", game.position.x, game.position.y);
    }
}

#[derive(Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
struct GameInfo {
    height: i32,
    width: i32,
    turn: i32,
    position: Position,
}

impl GameInfo {

    fn normalize(&mut self) {
        let w = self.width;
        let h = self.height;
        if self.position.x >= w {
            self.position.x = w - 1;
        } else if self.position.x < 0 {
            self.position.x = 0;
        }
        if self.position.y >= h {
            self.position.y = h - 1;
        } else if self.position.y < 0 {
            self.position.y = 0;
        }
    }

    fn update_position(&mut self, dir: &str) {
        match dir {
            "U" => self.position.y -= 1,
            "UR" => {
                self.position.x += 1;
                self.position.y -= 1;
            },
            "R" => self.position.x += 1,
            "DR" => {
                self.position.x += 1;
                self.position.y += 1;
            },
            "D" => self.position.y += 1,
            "DL" => {
                self.position.x -= 1;
                self.position.y += 1;
            },
            "L" => self.position.x -= 1,
            "UL" => {
                self.position.x -= 1;
                self.position.y -= 1;                
            },
            _ => unreachable!(),
        }

        self.normalize();
    } 
}

#[test]
fn test_false_up_direction() {
    let mut game: GameInfo = GameInfo { height: 2, width: 3, turn: 10, position: Position::default() };

    // We start at position (0,0);
    assert!(game.position.x == 0);
    assert!(game.position.y == 0);
   
    // try to go upper than building
    game.update_position("U");
    assert!(game.position.x == 0);
    assert!(game.position.y == 0);
}