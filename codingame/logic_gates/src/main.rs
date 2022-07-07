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

fn main() {
    let _input1_name: String = String::from("A");
    let _input1_signal: String = String::from("__---___---___---___---___");
    let _input2_name: String = String::from("B");
    let _input2_signal: String = String::from("____---___---___---___---_");
    let _output_name: String = String::from("C");
    let _output_type: String = String::from("AND");
    let _output_input1: String = String::from("A");
    let _output_input2: String = String::from("B");

    assert!(gate::and('_', '-') == '_');
    assert!(gate::or('-', '_') == '-');
    assert!(gate::xor('-', '-') == '_');
    assert!(gate::xor('-', '_') == '-');
    assert!(gate::nand('-', '-') == '_');
    assert!(gate::nor('-', '_') == '_');
    assert!(gate::nxor('-', '_') == '_');

    println!("done");
}
