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
 *
 * (0, 0) is the top left
 *
 *  ACTIONS:
 *      - MOVE x y
 *      - USE x y
 *          - on equipement you use it
 *          - on table with object (food or plates) without holding
 *            anything allow you to take the object
 *          - on dessert with a plate will load the dessert
 *      - WAIT: skip its turn
 *
 * customer item: DISH-BLUEBERRIES-CHOPPED_STRAWBERRIES  award: 850
 * customer item: DISH-CHOPPED_STRAWBERRIES-ICE_CREAM  award: 800
 * customer item: DISH-ICE_CREAM-CHOPPED_STRAWBERRIES-BLUEBERRIES  award: 1050
 * customer item: DISH-CROISSANT-BLUEBERRIES-ICE_CREAM  award: 1300
 * customer item: DISH-BLUEBERRIES-CHOPPED_STRAWBERRIES-CROISSANT  award: 1500
 * customer item: DISH-ICE_CREAM-CROISSANT  award: 1050
 * customer item: DISH-ICE_CREAM-CHOPPED_STRAWBERRIES  award: 800
 * customer item: DISH-CROISSANT-CHOPPED_STRAWBERRIES  award: 1250
 * customer item: DISH-CROISSANT-CHOPPED_STRAWBERRIES-ICE_CREAM  award: 1450
 * customer item: DISH-ICE_CREAM-CHOPPED_STRAWBERRIES  award: 800
 * customer item: DISH-ICE_CREAM-CROISSANT-BLUEBERRIES-CHOPPED_STRAWBERRIES  award: 1700
 * customer item: DISH-BLUEBERRIES-CHOPPED_STRAWBERRIES-ICE_CREAM-CROISSANT  award: 1700
 * customer item: DISH-CROISSANT-ICE_CREAM  award: 1050
 * customer item: DISH-BLUEBERRIES-CHOPPED_STRAWBERRIES-ICE_CREAM  award: 1050
 * customer item: DISH-ICE_CREAM-CHOPPED_STRAWBERRIES  award: 800
 * customer item: DISH-CHOPPED_STRAWBERRIES-CROISSANT  award: 1250
 * customer item: DISH-ICE_CREAM-CROISSANT-CHOPPED_STRAWBERRIES-BLUEBERRIES  award: 1700
 * customer item: DISH-ICE_CREAM-BLUEBERRIES  award: 650
 * customer item: DISH-BLUEBERRIES-CHOPPED_STRAWBERRIES-CROISSANT-ICE_CREAM  award: 1700
 * customer item: DISH-ICE_CREAM-CHOPPED_STRAWBERRIES  award: 800
 *
 * => States:
 * CHOPPED_STRAWBERRIES
 * CROISSANT
 * DISH
 * BLUEBERRIES
 * ICE_CREAM
 */

/********************************************************
 * Ref: https://fr.wikipedia.org/wiki/Automate_fini
 * - [X] Commencons par tester le "portillon" (FSM)
 * - [X] Le loup, la chèvre et le chou
 * - [X] Apply to the game
 ********************************************************/
use std::io;

const KITCHEN_X: usize = 11;
const KITCHEN_Y: usize = 7;
const WS_CHAR: char = '#';
const P0_CHAR: char = '0';
const P1_CHAR: char = '1';
const ES_CHAR: char = '.';
const DI_CHAR: char = 'D';
const WI_CHAR: char = 'W';
const BL_CHAR: char = 'B';
const IC_CHAR: char = 'I';
const ST_CHAR: char = 'S';
const DO_CHAR: char = 'H';
const CH_CHAR: char = 'C';
const OV_CHAR: char = 'O';

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

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum CustomerItem {
    DISH,
    CHOPPED_STRAWBERRIES,
    CROISSANT,
    DOUGH,
    STRAW,
    OVEN,
    BLUEBERRIES,
    ICE_CREAM,
    NONE,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    WorkSpace,
    EmptySpace,
    Dish,
    Window,
    Blue,
    Ice,
    Straw,
    Dough,
    Chop,
    Oven,
}

#[derive(Debug)]
struct GameState {
    kitchen: [[Tile; KITCHEN_X]; KITCHEN_Y],
    fsm: Vec<CustomerItem>,
    player_states: Vec<CustomerItem>,
    player_pos: Option<Position>,
    table_items: Vec<(Position, CustomerItem)>,
    oven_timer: usize,
    croissant_ready: bool,
}

impl GameState {
    fn new() -> Self {
        Self {
            kitchen: [[Tile::EmptySpace; KITCHEN_X]; KITCHEN_Y],
            fsm: Vec::new(),
            player_states: Vec::new(),
            player_pos: None,
            table_items: Vec::new(),
            oven_timer: 0,
            croissant_ready: false,
        }
    }

    fn str_to_customer_item(s: &str) -> CustomerItem {
        match s {
            "NONE" => CustomerItem::NONE,
            "CHOPPED_STRAWBERRIES" => CustomerItem::CHOPPED_STRAWBERRIES,
            "CROISSANT" => CustomerItem::CROISSANT,
            "DISH" => CustomerItem::DISH,
            "DOUGH" => CustomerItem::DOUGH,
            "BLUEBERRIES" => CustomerItem::BLUEBERRIES,
            "ICE_CREAM" => CustomerItem::ICE_CREAM,
            "STRAWBERRIES" => CustomerItem::STRAW,
            "OVEN" => CustomerItem::OVEN,
            _ => unreachable!(),
        }
    }

    // If the item is in the list of table items it returns its Position,
    // Otherwise it returns None
    fn item_on_table(&self, item: &CustomerItem) -> Option<Position> {
        for (pos, ci) in self.table_items.iter() {
            if item == ci {
                return Some(Position { x: pos.x, y: pos.y });
            }
        }

        None
    }

    fn position_on_table(&self, p: &Position) -> Option<CustomerItem> {
        for (pos, item) in self.table_items.iter() {
            if pos.x == p.x && pos.y == p.y {
                return Some(*item);
            }
        }

        None
    }

    // Find a free place on workspace
    fn get_position_around_us(&self) -> Vec<Position> {
        let mut v: Vec<Position> = Vec::new();
        let ppos = if let Some(p) = &self.player_pos {
            p
        } else {
            unreachable!()
        };

        // We check all possible values and only keep valid ones
        if ppos.x as i32 - 1 >= 0 && ppos.y as i32 - 1 >= 0 {
            v.push(Position {
                x: ppos.x - 1,
                y: ppos.y - 1,
            });
        }
        if ppos.y as i32 - 1 >= 0 {
            v.push(Position {
                x: ppos.x,
                y: ppos.y - 1,
            });
        }
        if ppos.x + 1 < KITCHEN_X && ppos.y as i32 - 1 >= 0 {
            v.push(Position {
                x: ppos.x + 1,
                y: ppos.y - 1,
            });
        }

        if ppos.x as i32 - 1 >= 0 {
            v.push(Position {
                x: ppos.x - 1,
                y: ppos.y,
            });
        }
        if ppos.x + 1 < KITCHEN_X {
            v.push(Position {
                x: ppos.x + 1,
                y: ppos.y,
            });
        }

        if ppos.x + 1 < KITCHEN_X && ppos.y as i32 - 1 >= 0 {
            v.push(Position {
                x: ppos.x + 1,
                y: ppos.y - 1,
            });
        }
        if ppos.x + 1 < KITCHEN_X {
            v.push(Position {
                x: ppos.x + 1,
                y: ppos.y,
            });
        }
        if ppos.x + 1 < KITCHEN_X && ppos.y + 1 < KITCHEN_Y {
            v.push(Position {
                x: ppos.x + 1,
                y: ppos.y + 1,
            });
        }

        v
    }

    fn get_empty_workspace_around_us(&self) -> Position {
        let pos_around = self.get_position_around_us();
        eprintln!("pos around {:?}", pos_around);
        for p in pos_around.iter() {
            if self.kitchen[p.y][p.x] == Tile::WorkSpace {
                match self.position_on_table(p) {
                    None => return Position { x: p.x, y: p.y },
                    _ => continue,
                }
            }
        }
        unreachable!()
    }

    // Takes a string like "DISH-ICE_CREAM-CHOPPED_STRAWBERRIES"
    // and generates states => [ICE_CREAM, DISH, CHOPPED_STRAWBERRIES]
    fn gen_vec_of_customer_item(s: &str) -> Vec<CustomerItem> {
        eprintln!("Str: {}", s);

        let mut v: Vec<CustomerItem> = s.split('-').map(GameState::str_to_customer_item).collect();
        // Now we sort to have complex preparation first
        v.sort_unstable();
        v.reverse(); // reverse because we will pop to get next state
        v
    }

    fn set_fsm(&mut self, s: &str) {
        self.fsm = GameState::gen_vec_of_customer_item(s);
        eprintln!("{:#?}", self.fsm);
    }

    // Set tile in the kitchen
    fn set_tile(&mut self, p: Position, c: char) {
        match c {
            WS_CHAR => self.kitchen[p.y][p.x] = Tile::WorkSpace,
            P0_CHAR => self.kitchen[p.y][p.x] = Tile::EmptySpace,
            P1_CHAR => self.kitchen[p.y][p.x] = Tile::EmptySpace,
            ES_CHAR => self.kitchen[p.y][p.x] = Tile::EmptySpace,
            DI_CHAR => self.kitchen[p.y][p.x] = Tile::Dish,
            WI_CHAR => self.kitchen[p.y][p.x] = Tile::Window,
            BL_CHAR => self.kitchen[p.y][p.x] = Tile::Blue,
            IC_CHAR => self.kitchen[p.y][p.x] = Tile::Ice,
            ST_CHAR => self.kitchen[p.y][p.x] = Tile::Straw,
            DO_CHAR => self.kitchen[p.y][p.x] = Tile::Dough,
            CH_CHAR => self.kitchen[p.y][p.x] = Tile::Chop,
            OV_CHAR => self.kitchen[p.y][p.x] = Tile::Oven,
            _ => todo!(),
        }
    }

    // Get the tile at a given position
    fn get_tile(&self, p: &Position) -> Tile {
        match self.kitchen[p.y][p.x] {
            Tile::WorkSpace => Tile::WorkSpace,
            Tile::EmptySpace => Tile::EmptySpace,
            Tile::Dish => Tile::Dish,
            Tile::Window => Tile::Window,
            Tile::Blue => Tile::Blue,
            Tile::Ice => Tile::Ice,
            Tile::Straw => Tile::Straw,
            Tile::Dough => Tile::Dough,
            Tile::Chop => Tile::Chop,
            Tile::Oven => Tile::Oven,
        }
    }

    // Get the position of a given item
    fn get_tile_position(&self, t: Tile) -> Position {
        for y in 0..KITCHEN_Y {
            for x in 0..KITCHEN_X {
                let pos = Position { x, y };
                if self.get_tile(&pos) == t {
                    return pos;
                }
            }
        }
        unreachable!();
    }

    fn display(&self) {
        for y in 0..KITCHEN_Y {
            for x in 0..KITCHEN_X {
                let c = match self.get_tile(&Position { x, y }) {
                    Tile::WorkSpace => WS_CHAR,
                    Tile::EmptySpace => ES_CHAR,
                    Tile::Dish => DI_CHAR,
                    Tile::Window => WI_CHAR,
                    Tile::Blue => BL_CHAR,
                    Tile::Ice => IC_CHAR,
                    Tile::Straw => ST_CHAR,
                    Tile::Dough => DO_CHAR,
                    Tile::Chop => CH_CHAR,
                    Tile::Oven => OV_CHAR,
                };
                eprint!("{}", c);
            }
            eprintln!("");
        }
    }

    fn get_action(&mut self) -> String {
        let mut current_state = if let Some(s) = self.fsm.last() {
            s
        } else {
            unreachable!()
        };

        eprintln!("Player state: {:?}", self.player_states);
        eprintln!("Current state: {:?}", *current_state);

        if self.player_states.contains(current_state) {
            // Go to next state
            self.fsm.pop();
            current_state = if let Some(s) = self.fsm.last() {
                s
            } else {
                &CustomerItem::NONE
            };
        }

        eprintln!("Updated current state: {:?}", *current_state);

        let msg: String = match *current_state {
            CustomerItem::NONE => "WAIT".to_string(),
            CustomerItem::DISH => {
                let dish_pos = self.get_tile_position(Tile::Dish);
                format!("USE {} {}", dish_pos.x, dish_pos.y)
            }
            CustomerItem::BLUEBERRIES => {
                let blueberry_pos = self.get_tile_position(Tile::Blue);
                format!("USE {} {}", blueberry_pos.x, blueberry_pos.y)
            }
            CustomerItem::ICE_CREAM => {
                let ice_cream_pos = self.get_tile_position(Tile::Ice);
                format!("USE {} {}", ice_cream_pos.x, ice_cream_pos.y)
            }
            CustomerItem::CHOPPED_STRAWBERRIES => {
                // Before cooking a croissant check that there is no croissant already
                // cooked.
                let index = self
                    .table_items
                    .iter()
                    .find(|(_, item)| item == &CustomerItem::CHOPPED_STRAWBERRIES);
                if let Some(index) = index {
                    format!("USE {} {}", index.0.x, index.0.y)
                } else {
                    // We need to hold STRAW before using chop
                    if self.player_states.contains(&CustomerItem::STRAW) {
                        let chop_pos = self.get_tile_position(Tile::Chop);
                        format!("USE {} {}", chop_pos.x, chop_pos.y)
                    } else {
                        let straw_pos = self.get_tile_position(Tile::Straw);
                        format!("USE {} {}", straw_pos.x, straw_pos.y)
                    }
                }
            }
            CustomerItem::CROISSANT => {
                // If we are currently cooking a croissant let's continue
                if self.player_states.contains(&CustomerItem::DOUGH) {
                    let oven_pos = self.get_tile_position(Tile::Oven);
                    format!("USE {} {}", oven_pos.x, oven_pos.y)
                } else {
                    // Before cooking a croissant check that there is no croissant already
                    // cooked.
                    let index = self
                        .table_items
                        .iter()
                        .find(|(_, item)| item == &CustomerItem::CROISSANT);
                    if let Some(index) = index {
                        format!("USE {} {}", index.0.x, index.0.y)
                    } else if self.oven_timer != 0 {
                        // Check if there is already something in preparation in the oven
                        self.croissant_ready = true; // we will take it when timer will be equal to 0
                        format!("WAIT")
                    } else if self.croissant_ready {
                        self.croissant_ready = false;
                        let oven_pos = self.get_tile_position(Tile::Oven);
                        format!("USE {} {}", oven_pos.x, oven_pos.y)
                    } else {
                        // At this point We need to cook one.
                        // So take the dough and then put it in the oven. But before make sure
                        // that we are not holding something. If we are holding something we need
                        // to put it on the workspace and add the step in the FSM.
                        if !self.player_states.contains(&CustomerItem::NONE) {
                            let top_state = if let Some(s) = self.fsm.pop() {
                                s
                            } else {
                                unreachable!()
                            };
                            self.fsm.append(&mut self.player_states);
                            self.fsm.push(top_state);

                            let pos = self.get_empty_workspace_around_us();
                            format!("USE {} {}", pos.x, pos.y)
                        } else {
                            let dough_pos = self.get_tile_position(Tile::Dough);
                            format!("USE {} {}", dough_pos.x, dough_pos.y)
                        }
                    }
                }
            }
            _ => {
                eprintln!("Unknown state {:?}", *current_state);
                todo!()
            }
        };

        msg
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
    let mut game = GameState::new();

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
        //customer item: DISH-BLUEBERRIES-ICE_CREAM  award: 650
    }

    /***********************************************************
     * GET INFORMATION ABOUT PLACEMENT OF OBJECTS
     */

    for y in 0..7 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let kitchen_line = input_line.trim_matches('\n').to_string();
        //eprintln!("kitchen line {}: {}", y, kitchen_line);
        for (x, c) in kitchen_line.chars().enumerate() {
            game.set_tile(Position { x, y }, c);
        }
    }

    // At this point the kitchen has been setup
    eprintln!("Window at {:?}", game.get_tile_position(Tile::Window));
    eprintln!("Blueberry at {:?}", game.get_tile_position(Tile::Blue));
    eprintln!("IceCream at {:?}", game.get_tile_position(Tile::Ice));
    eprintln!("Strawberry at {:?}", game.get_tile_position(Tile::Straw));
    eprintln!("Dough at {:?}", game.get_tile_position(Tile::Dough));
    eprintln!("Chop at {:?}", game.get_tile_position(Tile::Chop));
    eprintln!("Oven at {:?}", game.get_tile_position(Tile::Oven));

    game.display();

    /***********************************************************
     * MAIN LOOP
     */
    eprintln!("-------------------");
    eprintln!("Start the loop game");
    eprintln!("-------------------");

    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let _turns_remaining = parse_input!(input_line, i32);
        //eprintln!("Turns remaining: {}", turns_remaining);

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();

        // Update the position of the player
        game.player_pos = Some(Position {
            x: parse_input!(inputs[0], usize),
            y: parse_input!(inputs[1], usize),
        });

        game.player_states = GameState::gen_vec_of_customer_item(inputs[2].trim());
        eprintln!("Player state {:?}", game.player_states);

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
            let table_x = parse_input!(inputs[0], usize);
            let table_y = parse_input!(inputs[1], usize);
            let item = inputs[2].trim().to_string();
            game.table_items.push((
                Position {
                    x: table_x,
                    y: table_y,
                },
                GameState::str_to_customer_item(&item),
            ));
        }
        eprintln!("table_items: {:?}", game.table_items);

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let oven_contents = inputs[0].trim().to_string(); // ignore until wood 1 league
        game.oven_timer = parse_input!(inputs[1], usize);
        eprintln!("oven contents {}, timer {}", oven_contents, game.oven_timer);
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
                eprintln!("Take this one");
                max_award = customer_award;
                order = customer_item;
            }
        }

        eprintln!("selected order: {:#?}", order);
        // customer item: DISH-BLUEBERRIES-ICE_CREAM  award: 638

        // get state new order if empty
        if order.is_empty() {
            // Wait for a better order
            println!("WAIT");
            continue;
        }

        if game.fsm.is_empty() {
            // Deliver and take a new order
            if game.player_states.contains(&CustomerItem::NONE) {
                game.set_fsm(&order);
            } else {
                let win_pos = game.get_tile_position(Tile::Window);
                println!("USE {} {}", win_pos.x, win_pos.y);
                continue;
            }
        }

        game.display();

        eprintln!("FSM: {:#?}", game.fsm);

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
