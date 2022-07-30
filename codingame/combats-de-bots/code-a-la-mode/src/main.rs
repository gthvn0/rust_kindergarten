/*
 * Kitchen is 11 x 7
 *
 *  Here is how kitchen is modeled:
 *  kitchen line: #####D#####
 *  kitchen line: #..1......#
 *  kitchen line: #.####0##.#
 *  kitchen line: B.#..I..#.#
 *  kitchen line: #.##.####.#
 *  kitchen line: #.........#
 *  kitchen line: #####W#####
 * *
 *  ACTIONS:
 *      - MOVE x y
 *      - USE x y
 *          - on equipement you use it
 *          - on table with object (food or plates) without holding
 *            anything allow you to take the object
 *          - on dessert with a plate will load the dessert
 *      - WAIT: skip its turn
 */

/********************************************************
 * Ref: https://fr.wikipedia.org/wiki/Automate_fini
 * - [X] Commencons par tester le "portillon" (FSM)
 * - [X] Le loup, la chèvre et le chou
 * - [X] Apply to the game
 ********************************************************/
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Debug, Default, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum PlayerState {
    Dish,
    PutPlate,
    TakePlate,
    Ice,
    Straw,
    Chop,
    Dough,
    Oven,
    Blue,
    Deliver,
}

#[derive(Debug)]
struct GameState {
    states: Vec<PlayerState>, // current state is the last item
    empty_space: Vec<Position>,
    workspace: Vec<Position>,
    saved_workspace: Position,
    player: Position,
    dish: Position,
    window: Position,
    ice: Position,
    blue: Position,
    straw: Position,
    dough: Position,
    chop: Position,
    oven: Position,
}

//macro_rules! use_or_move {
//    ($self:ident, $item:ident) => {
//        let near_item: Position = $self.near_empty_space(&$self.$item);
//        let msg = if $self.player == near_item {
//            $self.states.pop();
//            format!("USE {} {}", $self.$item.x, $self.$item.y)
//        } else {
//            format!("MOVE {} {}", near_item.x, near_item.y)
//        };
//        msg
//    };
//}

impl GameState {
    // Takes a string like "DISH-ICE_CREAM-CHOPPED_STRAWBERRIES"
    // and generates states => [Deliver, Straw, Ice, Dish]
    fn set_states(&mut self, s: &String) {
        self.states = Vec::new();

        for item in s.split('-').into_iter().map(|s| s.to_uppercase()) {
            match &item[..] {
                "DISH" => self.states.push(PlayerState::Dish),
                "CROISSANT" => {
                    self.states.push(PlayerState::PutPlate);
                    self.states.push(PlayerState::Oven);
                    self.states.push(PlayerState::Dough);
                    self.states.push(PlayerState::TakePlate);
                }
                "CHOPPED_STRAWBERRIES" => {
                    self.states.push(PlayerState::PutPlate);
                    self.states.push(PlayerState::Straw);
                    self.states.push(PlayerState::Chop);
                    self.states.push(PlayerState::TakePlate);
                }
                "ICE_CREAM" => self.states.push(PlayerState::Ice),
                "BLUEBERRIES" => self.states.push(PlayerState::Blue),
                _ => unreachable!(),
            }
        }

        self.states.push(PlayerState::Deliver);
        // And now we just need to reverse it to get the right order
        self.states.reverse();
        eprintln!("{:#?}", self.states);
    }

    fn near_empty_space(&self, item: &Position) -> Position {
        // Return the first empty space near the item
        // There is height possible values
        let check_pos: [(usize, usize); 8] = [
            (item.x - 1, item.y - 1),
            (item.x, item.y - 1),
            (item.x + 1, item.y - 1),
            (item.x - 1, item.y),
            (item.x + 1, item.y),
            (item.x - 1, item.y + 1),
            (item.x, item.y + 1),
            (item.x + 1, item.y + 1),
        ];

        for (a, b) in check_pos.iter() {
            if self.empty_space.contains(&Position { x: *a, y: *b }) {
                return Position { x: *a, y: *b };
            }
        }
        // If we are here it means that we can not catch the item...
        unreachable!()
    }

    fn set_workspace(&mut self) -> bool {
        // Set saved_workspace to the first available workspace around us.
        let check_pos: [(usize, usize); 8] = [
            (self.player.x - 1, self.player.y - 1),
            (self.player.x, self.player.y - 1),
            (self.player.x + 1, self.player.y - 1),
            (self.player.x - 1, self.player.y),
            (self.player.x + 1, self.player.y),
            (self.player.x - 1, self.player.y + 1),
            (self.player.x, self.player.y + 1),
            (self.player.x + 1, self.player.y + 1),
        ];

        for (a, b) in check_pos.iter() {
            if self.workspace.contains(&Position { x: *a, y: *b }) {
                self.saved_workspace = Position { x: *a, y: *b };
                eprintln!("savec workspace at {} {}", *a, *b);
                return true;
            }
        }

        false
    }

    fn get_action(&mut self) -> String {
        match self.states.last() {
            Some(PlayerState::Dish) => {
                eprintln!("On va chercher une assiette");
                let near_dish: Position = self.near_empty_space(&self.dish);
                let msg = if self.player == near_dish {
                    self.states.pop();
                    format!("USE {} {}", self.dish.x, self.dish.y)
                } else {
                    format!("MOVE {} {}", near_dish.x, near_dish.y)
                };
                msg
            }
            Some(PlayerState::Ice) => {
                eprintln!("On va chercher l'icecream");
                let near_ice: Position = self.near_empty_space(&self.ice);
                let msg = if self.player == near_ice {
                    self.states.pop();
                    format!("USE {} {}", self.ice.x, self.ice.y)
                } else {
                    format!("MOVE {} {}", near_ice.x, near_ice.y)
                };
                msg
            }
            Some(PlayerState::Blue) => {
                eprintln!("On va chercher le blueberry");
                let near_blue: Position = self.near_empty_space(&self.blue);
                let msg = if self.player == near_blue {
                    self.states.pop();
                    format!("USE {} {}", self.blue.x, self.blue.y)
                } else {
                    format!("MOVE {} {}", near_blue.x, near_blue.y)
                };
                msg
            }
            Some(PlayerState::Straw) => {
                eprintln!("On va chercher la fraise et la couper");
                let near_straw: Position = self.near_empty_space(&self.straw);
                let msg = if self.player == near_straw {
                    self.states.pop();
                    format!("USE {} {}", self.straw.x, self.straw.y)
                } else {
                    format!("MOVE {} {}", near_straw.x, near_straw.y)
                };
                msg
            }
            Some(PlayerState::Chop) => {
                eprintln!("On va couper la fraise");
                let near_chop: Position = self.near_empty_space(&self.chop);
                let msg = if self.player == near_chop {
                    self.states.pop();
                    format!("USE {} {}", self.chop.x, self.chop.y)
                } else {
                    format!("MOVE {} {}", near_chop.x, near_chop.y)
                };
                msg
            }
            Some(PlayerState::Deliver) => {
                eprintln!("On donne au client");
                let near_window: Position = self.near_empty_space(&self.window);
                let msg = if self.player == near_window {
                    self.states.pop();
                    format!("USE {} {}", self.window.x, self.window.y)
                } else {
                    format!("MOVE {} {}", near_window.x, near_window.y)
                };
                msg
            }
            Some(PlayerState::TakePlate) => {
                eprintln!("On recupere l'assiette sur le plan de travail");
                let near_saved_workspace: Position = self.near_empty_space(&self.saved_workspace);
                let msg = if self.player == near_saved_workspace {
                    self.states.pop();
                    format!("USE {} {}", self.saved_workspace.x, self.saved_workspace.y)
                } else {
                    format!("MOVE {} {}", near_saved_workspace.x, near_saved_workspace.y)
                };
                msg
            }

            Some(PlayerState::PutPlate) => {
                eprintln!("On pose l'assiette sur le plan de travail");
                let msg = if self.set_workspace() {
                    self.states.pop();
                    format!("USE {} {}", self.saved_workspace.x, self.saved_workspace.y)
                } else {
                    unreachable!()
                };
                msg
            }
            Some(PlayerState::Oven) => {
                eprintln!("On fait cuire");
                let near_oven: Position = self.near_empty_space(&self.oven);
                let msg = if self.player == near_oven {
                    self.states.pop();
                    format!("USE {} {}", self.oven.x, self.oven.y)
                } else {
                    format!("MOVE {} {}", near_oven.x, near_oven.y)
                };
                msg
            }
            Some(PlayerState::Dough) => {
                eprintln!("On va chercher la pate");
                let near_dough: Position = self.near_empty_space(&self.dough);
                let msg = if self.player == near_dough {
                    self.states.pop();
                    format!("USE {} {}", self.dough.x, self.dough.y)
                } else {
                    format!("MOVE {} {}", near_dough.x, near_dough.y)
                };
                msg
            }
            _ => todo!(),
        }
    }
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    eprintln!("-------------------");
    eprintln!("Init informations");
    eprintln!("-------------------");

    /***
     * Game State
     */
    let mut game = GameState {
        states: Vec::new(),
        empty_space: Vec::new(),
        workspace: Vec::new(),
        // Put item in inaccessible position for init
        saved_workspace: Position { x: 42, y: 42 },
        player: Position { x: 42, y: 42 },
        dish: Position { x: 42, y: 42 },
        window: Position { x: 42, y: 42 },
        blue: Position { x: 42, y: 42 },
        ice: Position { x: 42, y: 42 },
        straw: Position { x: 42, y: 42 },
        dough: Position { x: 42, y: 42 },
        chop: Position { x: 42, y: 42 },
        oven: Position { x: 42, y: 42 },
    };

    /***********************************************************
     * INFORMATION ABOUT CUSTOMERS
     *
     * Not used for now
     */
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num_all_customers = parse_input!(input_line, i32);
    eprintln!("num_all_customers: {}", num_all_customers);
    // num_all_customers: 20

    for _ in 0..num_all_customers as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let customer_item = inputs[0].trim().to_string(); // the food the customer is waiting for
        let customer_award = parse_input!(inputs[1], i32); // the number of points awarded for delivering the food
        eprintln!(
            "customer item: {}  award: {}",
            customer_item, customer_award
        );
        // customer item: DISH-BLUEBERRIES-ICE_CREAM  award: 650
    }

    /***********************************************************
     * GET INFORMATION ABOUT PLACEMENT OF OBJECTS
     */

    for y in 0..7 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let kitchen_line = input_line.trim_matches('\n').to_string();
        eprintln!("kitchen line {}: {}", y, kitchen_line);
        for (x, c) in kitchen_line.chars().enumerate() {
            match c {
                '#' => game.workspace.push(Position { x, y }),
                '0' => game.empty_space.push(Position { x, y }),
                '1' => game.empty_space.push(Position { x, y }),
                '.' => game.empty_space.push(Position { x, y }),
                'D' => game.dish = Position { x, y },
                'W' => game.window = Position { x, y },
                'B' => game.blue = Position { x, y },
                'I' => game.ice = Position { x, y },
                'S' => game.straw = Position { x, y },
                'H' => game.dough = Position { x, y },
                'C' => game.chop = Position { x, y },
                'O' => game.oven = Position { x, y },
                _ => continue,
            }
        }
    }

    eprintln!("Dish at {:?}", game.dish);
    eprintln!("Window at {:?}", game.window);
    eprintln!("Blueberry at {:?}", game.blue);
    eprintln!("IceCream at {:?}", game.ice);
    eprintln!("Strawberry at {:?}", game.straw);
    eprintln!("Dough at {:?}", game.dough);
    eprintln!("Chop at {:?}", game.chop);
    eprintln!("Oven at {:?}", game.oven);

    /***********************************************************
     * MAIN LOOP
     */
    eprintln!("-------------------");
    eprintln!("Start the loop game");
    eprintln!("-------------------");

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let turns_remaining = parse_input!(input_line, i32);
        eprintln!("Turns remaining: {}", turns_remaining);
        // Turns remaining: 187

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        game.player = Position {
            x: parse_input!(inputs[0], usize),
            y: parse_input!(inputs[1], usize),
        };

        let player_state = inputs[2].trim();
        eprintln!("state == {}", player_state);

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let partner_x = parse_input!(inputs[0], i32);
        let partner_y = parse_input!(inputs[1], i32);
        let partner_item = inputs[2].trim().to_string();
        eprintln!(
            "partner ({}, {}), item {}",
            partner_x, partner_y, partner_item
        );
        // partner (1, 1), item DISH-BLUEBERRIES

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_tables_with_items = parse_input!(input_line, i32); // the number of tables in the kitchen that currently hold an item
        eprintln!("num tables with item: {}", num_tables_with_items);
        // num tables with item: 0

        for _ in 0..num_tables_with_items as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let table_x = parse_input!(inputs[0], i32);
            let table_y = parse_input!(inputs[1], i32);
            let item = inputs[2].trim().to_string();
            eprintln!("table ({}, {}), item {}", table_x, table_y, item);
            // table (9, 6), item DISH
        }

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let oven_contents = inputs[0].trim().to_string(); // ignore until wood 1 league
        let oven_timer = parse_input!(inputs[1], i32);
        eprintln!("oven contents {}, timer {}", oven_contents, oven_timer);
        // oven contents NONE, timer 0

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let num_customers = parse_input!(input_line, i32); // the number of customers currently waiting for food
        eprintln!("number of customers waiting for food {}", num_customers);
        // number of customers waiting for food 3

        let mut max_award: i32 = 0;
        let mut order = String::new();
        for _ in 0..num_customers as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let customer_item = inputs[0].trim().to_string();
            let customer_award = parse_input!(inputs[1], i32);
            eprintln!("customer item: {} award: {}", customer_item, customer_award);
            if customer_award > max_award {
                max_award = customer_award;
                order = customer_item;
            }
        }
        eprintln!("items: {}", order);
        // customer item: DISH-BLUEBERRIES-ICE_CREAM  award: 638

        // get state new order if empty
        if game.states.is_empty() {
            game.set_states(&order);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // MOVE x y
        // USE x y
        // WAIT

        eprintln!("Player: {:?}", game);
        eprintln!("Your action...");
        println!("{}", game.get_action());
    }
}
