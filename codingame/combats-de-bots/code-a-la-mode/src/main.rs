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

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    eprintln!("-------------------");
    eprintln!("Init informations");
    eprintln!("-------------------");

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let num_all_customers = parse_input!(input_line, i32);
    eprintln!("num_all_customers: {}", num_all_customers);
    // num_all_customers: 20

    for i in 0..num_all_customers as usize {
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

    for i in 0..7 as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let kitchen_line = input_line.trim_matches('\n').to_string();
        eprintln!("kitchen line {}: {}", i, kitchen_line);
        // kitchen line 0: #####D####I
    }

    let mut commands: Vec<&str> = vec!["MOVE 9 5"];

    commands.reverse(); // Put order ready for poping in the game loop

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
        let player_x = parse_input!(inputs[0], i32);
        let player_y = parse_input!(inputs[1], i32);
        let player_item = inputs[2].trim().to_string();
        eprintln!("player ({}, {}), item {}", player_x, player_y, player_item);
        // player (5, 1), item DISH

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

        for i in 0..num_tables_with_items as usize {
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

        for i in 0..num_customers as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let customer_item = inputs[0].trim().to_string();
            let customer_award = parse_input!(inputs[1], i32);
            eprintln!(
                "customer item: {}  award: {}",
                customer_item, customer_award
            );
            // customer item: DISH-BLUEBERRIES-ICE_CREAM  award: 638
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // MOVE x y
        // USE x y
        // WAIT

        eprintln!("Your action...");
        if let Some(cmd) = commands.pop() {
            println!("{}", cmd);
        } else {
            println!("WAIT");
        }
    }
}