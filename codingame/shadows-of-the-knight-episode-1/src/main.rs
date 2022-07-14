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
    let width = parse_input!(inputs[0], i32); // width of the building.
    let height = parse_input!(inputs[1], i32); // height of the building.

    // Set the bomb area
    game.bomb.top_left = Position {x:0, y:0};
    game.bomb.top_right = Position {x:width, y:0};
    game.bomb.bottom_left = Position {x:0, y:height};
    game.bomb.bottom_right = Position {x:width, y:height};

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    game.turn = parse_input!(input_line, i32); // maximum number of turns before game over.

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    game.position.x = parse_input!(inputs[0], i32);
    game.position.y = parse_input!(inputs[1], i32);

    // game loop    
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let bomb_dir = input_line.trim().to_string(); // the direction of the bombs from batman's current location (U, UR, R, DR, D, DL, L or UL)

        eprintln!("GameInfo {:?}", game);
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
struct BombArea {
    top_right: Position,
    top_left: Position,
    bottom_right: Position,
    bottom_left: Position,
}

#[derive(Debug, Default)]
struct GameInfo {
    bomb: BombArea,
    turn: i32,
    position: Position,
}

impl GameInfo {
    
    // Update the bomb area according to the next location
    //
    // # Arguments
    // * `dir` - The direction
    fn update_bomb_area(&mut self, dir: &str) {
        match dir {
            "U" => { // All things below us can be exclude
                self.bomb.bottom_left.y = self.position.y;
                self.bomb.bottom_right.y = self.position.y;
            },
            "UR" => { // Below and on the right
                self.bomb.bottom_left.y = self.position.y;
                self.bomb.bottom_right.y = self.position.y;
                self.bomb.top_left.x = self.position.x;
                self.bomb.bottom_left.x = self.position.x;
            },
            "R" => {
              self.bomb.top_left.x = self.position.x;
              self.bomb.bottom_left.x = self.position.x;  
            },
            "DR" => {
                self.bomb.top_right.y = self.position.y;
                self.bomb.top_left.y = self.position.y;
                self.bomb.top_left.x = self.position.x;
                self.bomb.bottom_left.x = self.position.x;
            },
            "D" => {
                self.bomb.top_right.y = self.position.y;
                self.bomb.top_left.y = self.position.y;        
            },
            "DL" => {
                self.bomb.top_right.y = self.position.y;
                self.bomb.top_left.y = self.position.y;        
                self.bomb.top_right.x = self.position.x;
                self.bomb.bottom_right.x = self.position.x;
            },
            "L" => {
                self.bomb.top_right.x = self.position.x;
                self.bomb.bottom_right.x = self.position.x;
            },
            "UL" => {
                self.bomb.bottom_left.y = self.position.y;        
                self.bomb.bottom_right.y = self.position.y;     
                self.bomb.top_right.x = self.position.x;
                self.bomb.bottom_right.x = self.position.x;
            },
            _ => unreachable!(),
        }
    }

    fn update_position(&mut self, dir: &str) {
        self.update_bomb_area(dir);
        match dir {
            "U" => {
                self.position.y = (self.position.y + self.bomb.top_left.y) / 2;
            },
            "UR" => {
                self.position.y = (self.position.y + self.bomb.top_left.y) / 2;
                self.position.x = (self.bomb.top_right.x + self.position.x) / 2;
            },
            "R" => {
                self.position.x = (self.bomb.top_right.x + self.position.x) / 2;
            },
            "DR" => {
                self.position.x = (self.bomb.bottom_right.x + self.position.x) / 2;
                self.position.y = (self.position.y + self.bomb.bottom_right.y) / 2;
            },
            "D" => {
                self.position.y = (self.bomb.bottom_left.y + self.position.y) / 2;
            },
            "DL" => {
                self.position.x = (self.position.x + self.bomb.bottom_left.x) / 2;
                self.position.y = (self.position.y + self.bomb.bottom_left.y) / 2;
            },
            "L" => {
                self.position.x = (self.bomb.bottom_left.x + self.position.x) / 2;
            },
            "UL" => {
                self.position.x = (self.bomb.top_left.y + self.position.y) / 2;
                self.position.y = (self.bomb.top_left.x + self.position.x) / 2;
            },
            _ => unreachable!(),
        }
    } 
}