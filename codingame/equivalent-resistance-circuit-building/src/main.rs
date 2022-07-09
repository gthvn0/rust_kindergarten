use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

/**
 * Tokens: '(', ')', '[', ']', Id
 *
 *     E -> S E | P E | R
 *     S -> ( E )
 *     P -> [ E ]
 *     R -> Id
 *
 * [ ( A B ) [ C A ] ]
 *
 *                E -> S + 1/P => (A + B) + 1/(1/C + 1/A)
 *               / \
 *              /   \
 *   A + B <-  S     P -> 1/C + 1/A
 *            / \   / \
 *           A  B  C   A
 *
 * [ A B C ], A=12, B=4, C=9 => 2.25
 *
 *       E -> 1/( 1/A + 1/B + 1/C)
 *       |
 *       P -> 1/A + (1/B + 1/C)
 *      / \
 *     /   P -> 1/B + 1/C
 *    A   / \
 *       B   C
 *
 *   
 */

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);

    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let name = inputs[0].trim().to_string();
        let r = parse_input!(inputs[1], i32);
    }

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let circuit = input_line.trim_matches('\n').to_string();

    // Write an answer using println!("message...");
    // To debug: eprintln!("Debug message...");
    // Example:
    // 3
    // A 24
    // B 8
    // C 48
    //
    // ==> Should return 10.7

    // ==> Equivalent to
    //
    //     +---[C]---+
    //     |         |
    //  +--+         +--+
    //  |  |         |  |
    //  |  +---[A]---+  |
    //  |               |
    //  +---[A]---[B]---+
    //  |               |
    //  +---[Battery]---+
    //
    // [ ( A B ) [ C A ] ] => [ 24+8 1/(1/48+1/24) ] => [ 32 16 ] => 1/(1/32+1/16) => 32/3 => 10.666... => 10.7
    //
    eprintln!("---[ Debug ]---");
    eprintln!("circuit: {}", circuit);

    println!("Equivalent Resistance");
}
