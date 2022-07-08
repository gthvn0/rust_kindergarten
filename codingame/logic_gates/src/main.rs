/*
 * Here is the kind of inputs:
 *
 * input_name: A
 * input_signal: __---___---___---___---___
 * input_name: B
 * input_signal: ____---___---___---___---_
 *
 * Inputs are between 1 and 4 (included).
 * We got "String"
 * And we need to produce outputs like:
 *
 * output_name: C
 * type: AND
 * input_name1: A
 * input_name2: B
 *
 * So here the expected output is a string that represents an AND
 * between A and B where '_' is 0 and '-' is 1.
 *
 * The number of outputs is between 1 and 16 (included).
 *
 * Possible gates are: AND, OR, XOR, NAND, NOR, NXOR
 */
use std::collections::HashMap;
use std::iter::zip;

mod gate {
    // Keep this one private for now
    fn not(a: char) -> char {
        match a {
            '-' => '_',
            _ => '-',
        }
    }

    pub fn and(a: char, b: char) -> char {
        match (a, b) {
            ('-', '-') => '-',
            _ => '_',
        }
    }

    pub fn or(a: char, b: char) -> char {
        match (a, b) {
            ('_', '_') => '_',
            _ => '-',
        }
    }

    pub fn xor(a: char, b: char) -> char {
        match (a, b) {
            ('_', '_') => '_',
            ('-', '-') => '_',
            _ => '-',
        }
    }

    pub fn nand(a: char, b: char) -> char {
        not(and(a, b))
    }

    pub fn nor(a: char, b: char) -> char {
        not(or(a, b))
    }

    pub fn nxor(a: char, b: char) -> char {
        not(xor(a, b))
    }
}

enum GateType {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    NXOR,
}

/*
 * To generate a new signal we take to signals as input that
 * are strings, we generate a tuple by zipping then and apply
 * the corresponding gate by mapping it. The macro is generating
 * a new iterator that contains the new signal.
 */

fn gen_signal(s1: &str, s2: &str, op: fn(char, char) -> char) -> String {
    zip(s1.chars(), s2.chars()).map(|(x, y)| op(x, y)).collect()
}

fn print_signal(h: &HashMap<String, String>, key: &String) {
    match h.get(key) {
        Some(s) => println!("{}: {}", key, s),
        _ => println!("No signal found"),
    }
}

fn print_gen_signal(
    h: &HashMap<String, String>,
    s1: &String,
    s2: &String,
    op: fn(char, char) -> char,
) {
    match (h.get(s1), h.get(s2)) {
        (Some(s1), Some(s2)) => println!("S: {}", gen_signal(s1, s2, op)),
        _ => panic!("Signals not found"),
    }
}

#[macro_export]
macro_rules! generate_signal {
    ($h:expr, $in1: expr, $in2: expr, $gate: expr) => {
        let (ops, opf): (&str, fn(char, char) -> char) = match $gate {
            GateType::AND => ("AND", gate::and),
            GateType::OR => ("OR", gate::or),
            GateType::XOR => ("XOR", gate::xor),
            GateType::NAND => ("NAND", gate::nand),
            GateType::NOR => ("NOR", gate::nor),
            GateType::NXOR => ("NXOR", gate::nxor),
        };

        println!("");
        print_signal($h, $in1);
        println!("{}", ops);
        print_signal($h, $in2);
        println!("==");
        print_gen_signal($h, $in1, $in2, opf);
    };
}

fn main() {
    let mut signals: HashMap<String, String> = HashMap::new();

    signals.insert(
        String::from("A"),
        String::from("__---___---___---___---___"),
    );
    signals.insert(
        String::from("B"),
        String::from("____---___---___---___---_"),
    );

    println!("Inputs signals are:");
    print_signal(&signals, &String::from("A"));
    print_signal(&signals, &String::from("B"));

    let in1: String = String::from("A");
    let in2: String = String::from("B");

    generate_signal!(&signals, &in1, &in2, GateType::AND);
    generate_signal!(&signals, &in1, &in2, GateType::OR);
    generate_signal!(&signals, &in1, &in2, GateType::XOR);
    generate_signal!(&signals, &in1, &in2, GateType::NAND);
    generate_signal!(&signals, &in1, &in2, GateType::NOR);
    generate_signal!(&signals, &in1, &in2, GateType::NXOR);
}
