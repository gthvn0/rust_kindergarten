/*
 * Kitchen is 11 x 7
 * So if we start by numbering from top, left
 * we will have cells from:
 *    0 -> 10
 *   11 -> 21
 *   22 -> 32
 *   33 -> 43
 *   44 -> 54
 *   55 -> 65
 *   66 -> 76
 *  => 77 cells
 *
 *  Here is how kitchen is modeled:
 *  kitchen line: #####D#####
 *  kitchen line: #..1......#
 *  kitchen line: #.####0##.#
 *  kitchen line: B.#..I..#.#
 *  kitchen line: #.##.####.#
 *  kitchen line: #.........#
 *  kitchen line: #####W#####
 *
 *  where:
 *      - '#': workspace
 *      - '.': empty case
 *      - '0': player 0
 *      - '1': player 1
 *      - 'D': dishwasher
 *      - 'W': window for client orders
 *      Options:
 *      - 'B': blueberries
 *      - 'I': ice cream
 *
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
 * - [ ] Apply to the game
 *
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
enum PState {
    None,
    Dish,
    DishIce,
    DishBlue,
    DishIceBlue,
}

#[derive(Debug)]
struct PlayerState {
    state: PState,
    player: Position,
    dish: Position,
    window: Position,
    ice: Position,
    blue: Position,
}

impl PlayerState {
    fn near(item: &Position) -> Position {
        // p1 is near
        let x: usize = match item.x {
            0 => 1,
            10 => 9,
            _ => item.x,
        };
        let y: usize = match item.y {
            0 => 1,
            6 => 5,
            _ => item.y,
        };

        Position { x, y }
    }

    fn get_action(&self) -> String {
        match self.state {
            PState::None => {
                // On va chercher une assiette
                let near_dish = PlayerState::near(&self.dish);
                let msg = if self.player == near_dish {
                    format!("USE {} {}", self.dish.x, self.dish.y)
                } else {
                    format!("MOVE {} {}", near_dish.x, near_dish.y)
                };
                msg
            }
            PState::Dish => {
                // On va chercher le bluebeary
                let near_blue: Position = PlayerState::near(&self.blue);
                let msg = if self.player == near_blue {
                    format!("USE {} {}", self.blue.x, self.blue.y)
                } else {
                    format!("MOVE {} {}", near_blue.x, near_blue.y)
                };
                msg
            }
            PState::DishBlue => {
                // On va chercher l'icecream
                let near_ice: Position = PlayerState::near(&self.ice);
                let msg = if self.player == near_ice {
                    format!("USE {} {}", self.ice.x, self.ice.y)
                } else {
                    format!("MOVE {} {}", near_ice.x, near_ice.y)
                };
                msg
            }
            PState::DishIce => {
                // On va chercher le bluebeary
                let near_blue: Position = PlayerState::near(&self.blue);
                let msg = if self.player == near_blue {
                    format!("USE {} {}", self.blue.x, self.blue.y)
                } else {
                    format!("MOVE {} {}", near_blue.x, near_blue.y)
                };
                msg
            }
            PState::DishIceBlue => {
                // On donne au client
                let near_window: Position = PlayerState::near(&self.window);
                let msg = if self.player == near_window {
                    format!("USE {} {}", self.window.x, self.window.y)
                } else {
                    format!("MOVE {} {}", near_window.x, near_window.y)
                };
                msg
            }
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
     * Player State: It is more the state of the Game
     */
    let mut player = PlayerState {
        state: PState::None,
        // Put item in inaccessible position for init
        player: Position { x: 42, y: 42 },
        dish: Position { x: 42, y: 42 },
        window: Position { x: 42, y: 42 },
        blue: Position { x: 42, y: 42 },
        ice: Position { x: 42, y: 42 },
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
                'D' => player.dish = Position { x, y },
                'W' => player.window = Position { x, y },
                'B' => player.blue = Position { x, y },
                'I' => player.ice = Position { x, y },
                _ => continue,
            }
        }
    }

    eprintln!("Dish at {:?}", player.dish);
    eprintln!("Window at {:?}", player.window);
    eprintln!("Blueberry at {:?}", player.blue);
    eprintln!("IceCream at {:?}", player.ice);

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

        player.player = Position {
            x: parse_input!(inputs[0], usize),
            y: parse_input!(inputs[1], usize),
        };

        let player_state = inputs[2].trim();
        eprintln!("state == {}", player_state);
        player.state = match player_state {
            "NONE" => PState::None,
            "DISH" => PState::Dish,
            "DISH-BLUEBERRIES" => PState::DishBlue,
            "DISH-ICE_CREAM" => PState::DishIce,
            "DISH-ICE_CREAM-BLUEBERRIES" => PState::DishIceBlue,
            "DISH-BLUEBERRIES-ICE_CREAM" => PState::DishIceBlue,
            _ => unreachable!(),
        };

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

        for _ in 0..num_customers as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let customer_item = inputs[0].trim().to_string();
            let customer_award = parse_input!(inputs[1], i32);
            eprintln!("customer item: {} award: {}", customer_item, customer_award);
            // customer item: DISH-BLUEBERRIES-ICE_CREAM  award: 638
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // MOVE x y
        // USE x y
        // WAIT

        eprintln!("Player: {:?}", player);
        eprintln!("Your action...");
        println!("{}", player.get_action());
    }
}
