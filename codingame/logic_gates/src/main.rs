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
 */
fn main() {
    println!("Hello, world!");
}
