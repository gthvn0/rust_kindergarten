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

/********************************************************
 * Ref: https://fr.wikipedia.org/wiki/Automate_fini
 * - [X] Commencons par tester le "portillon" (FSM)
 * - [ ] Le loup, la chèvre et le chou
 * - [ ] Apply to the game
 *
 ********************************************************
 * Probleme du loup, de la chevre et du chou
 *
 * P: Passeur
 * C: Chevre
 * L: Loup
 * S: Chou
 *
 * - L'état initial est celui ou de départ ou tout le monde
 *   est sur l'ile: PCLS
 * - L'état final est celui ou l'ile est VIDE
 * - 'x' est un transition impossible.
 *
 *        +------+------+--------+------+
 *        | Seul | Loup | Chevre | Chou |
 * +------+------+------+--------+------+
 * | PCLS | x    | x    | ls     |  x   | => On ne peut pas laisser la chevre avec chou || loup
 * | pls  | ls   | s    | x      |  l   |
 * | pcs  | x    | x    | s      |  c   |
 * | plc  | x    | c    | l      |  x   |
 * | ls   | pls  | x    | pcls   |  x   |
 * | ps   | s    | x    | x      | VIDE |
 * | pc   | c    | x    | VIDE   |  x   |
 * | pl   | l    | VIDE | x      |  x   |
 * | s    | ps   | pls  | pcs    |  x   |
 * | l    | pl   | x    | plc    | pls  |
 * | c    | pc   | pcl  | x      | pcs  |
 * +------+------+------+--------+------+
 *
 *******************************************************
 */

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum State {
    PCLS, // Initial State
    PLS,
    PCS,
    PLC,
    PS,
    PC,
    PL,
    LS,
    S,
    L,
    C,
    Failed,
    Vide, // Final state
}

enum Event {
    Seul,
    Loup,
    Chevre,
    Chou,
}

#[derive(Debug)]
struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine { state: State::PCLS }
    }

    fn display(&self) {
        println!("State machine: {:?}", self.state);
    }

    fn apply_event(&mut self, _ev: Event) {
        todo!();
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
            "SEUL" => Event::Seul,
            "LOUP" => Event::Loup,
            "CHEVRE" => Event::Chevre,
            "CHOU" => Event::Chou,
            _ => {
                println!("Event can be: Seul, Loup, chevre or chou");
                continue;
            }
        };

        fsm.apply_event(ev);
        fsm.display();
    }
}
