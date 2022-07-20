use std::io::{self, Write};

/********************************************************
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
 * | pcl  | x    | c    | l      |  x   |
 * | pc   | c    | x    | VIDE   |  x   |
 * | ls   | pls  | x    | pcls   |  x   |
 * | s    | x    | pls  | pcs    |  x   | => PS n'est pas valide car il implique que la Chevre et
 * | l    | x    | x    | plc    | pls  |    le loup sont seuls sur l'autre ile (comme PL)
 * | c    | pc   | plc  | x      | pcs  |
 * +------+------+------+--------+------+
 *
 *******************************************************/

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    PCLS, // Initial State
    PLS,
    PCS,
    PCL,
    PC,
    LS,
    S,
    L,
    C,
    Err,
    Vide, // Final state
}

#[derive(Debug)]
enum Event {
    Seul,
    Loup,
    Chevre,
    Chou,
}

#[derive(Debug)]
struct StateMachine {
    state: State,
    // Vec of vec for all transition where a vec for a given transition is as follow:
    //     => PCLS = [ x,  x, ls,  x]
    transitions: Vec<Vec<State>>,
}

impl StateMachine {
    fn new() -> Self {
        // TODO: How can it be a little bit more automatic...
        //       provide a method to add a transition instead...
        StateMachine {
            state: State::PCLS,
            transitions: vec![
                     // seul       loup       chevre        chou
                vec![State::Err, State::Err, State::LS  , State::Err], // PCLS
                vec![State::LS , State::S  , State::Err , State::L  ], // PLS
                vec![State::Err, State::Err, State::S   , State::C  ], // PCS
                vec![State::Err, State::C  , State::L   , State::Err], // PCL
                vec![State::C  , State::Err, State::Vide, State::Err], // PC
                vec![State::PLS, State::Err, State::PCLS, State::Err], // LS
                vec![State::Err, State::PLS, State::PCS , State::Err], // S
                vec![State::Err, State::Err, State::PCL , State::PLS], // L
                vec![State::PC , State::PCL, State::Err , State::PCS], // C
            ]
         }
    }

    fn display(&self) {
        println!("FSM: {:?}", self);
    }

    fn get_state(&self) -> State {
        self.state
    }

    fn apply_event(&mut self, ev: Event) {
        let new_state: State = self.transitions[self.state as usize][ev as usize];

        match new_state {
            State::Err => println!("Oulala c'est pas possible ça"),
            _ => self.state = new_state,
        }
    }

}

fn main() {
    let mut fsm: StateMachine = StateMachine::new();

    fsm.display();

    println!("Regles du jeu:");
    println!("Les actions possibles sont:");
    println!("  - seul: vous traversez seul");
    println!("  - loup: vous traversez avec le loup");
    println!("  - chevre: vous traversez avec la chevre");
    println!("  - chou: vous traversez avec le chou");
    println!("Vous devez faire traverser le loup, la chevre et le chou sur la rive.");
    println!("Vous ne pouvez transporter qu'une seule chose a la fois");
    println!("Et attention !!!");
    println!("Le loup ne doit pas être SEUL avec la chevre, il la mangerait...");
    println!("La chevre ne doit pas être SEUL avec le chou, elle le mangerait...");
    println!("");

    loop {
        // infinite loop

        match fsm.get_state() {
            State::PCLS => println!("Vous êtes sur l'ile avec le loup, la chevre et le chou"),
            State::PLS  => println!("Vous etes sur l'ile avec le loup et le chou, la chevre est de l'autre coté"),
            State::PCS  => println!("Vous etes sur l'ile avec la chevre et le chou, le loup est de l'autre coté"),
            State::PCL  => println!("Vous etes sur l'ile avec la chevre et le loup, le chou est de l'autre coté"),
            State::PC   => println!("Vous etes sur l'ile avec la chevre, le loup et le chou sont de l'autre coté"),
            State::LS   => println!("Le loup et le chou sont sur l'ile, vous êtes de l'autre coté avec la chevre"),
            State::S    => println!("Le chou est sur l'ile, vous êtes de l'autre coté avec la chevre et le loup"),
            State::L    => println!("Le loup est sur l'ile, vous êtes de l'autre coté avec la chevre et le chou"),
            State::C    => println!("La chevre est sur l'ile, vous êtes de l'autre coté avec le loup et le chou"),
            State::Err  => unreachable!(),
            State::Vide => {
                println!("L'ile est vide... VICTOIRE !!!");
                break
            },
        }

        let mut input_line = String::new();
        print!("votre action (seul, loup, chevre, chou): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_line).unwrap();

        let input = input_line.trim().to_string().to_uppercase();

        let ev: Event = match &input as &str {
            "SEUL" => Event::Seul,
            "LOUP" => Event::Loup,
            "CHEVRE" => Event::Chevre,
            "CHOU" => Event::Chou,
            _ => {
                println!("Selectionnez votre action en ecrivant: seul, loup, chevre, chou. ");
                continue;
            }
        };

        fsm.apply_event(ev);
    }
}
