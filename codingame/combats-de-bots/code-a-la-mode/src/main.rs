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
use std::io;

// Ref: https://fr.wikipedia.org/wiki/Automate_fini
// - [X] Commencons par tester le "portillon" (FSM)
// - [ ] Le loup, la chèvre et le chou
// - [ ] Apply to the game

#[derive(Debug, PartialEq)]
enum State {
    Locked,
    Unlocked,
}

enum Event {
    Coin,
    Push,
}

#[derive(Debug)]
struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::Locked,
        }
    }

    fn display(&self) {
        println!("State machine: {:?}", self.state);
    }

    fn apply_event(&mut self, ev: Event) {
        self.state = match ev {
            Event::Push => State::Locked,
            Event::Coin => State::Unlocked,
        };
    }
}

fn main() {
    let mut fsm: StateMachine = StateMachine::new();

    fsm.display();

    loop {
        // infinite loop
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();

        let input = input_line.trim().to_string().to_uppercase();

        let ev: Event = match &input as &str {
            "PUSH" => Event::Push,
            "COIN" => Event::Coin,
            _ => {
                println!("Not a valid input");
                continue;
            }
        };

        fsm.apply_event(ev);
        fsm.display();
    }
}
