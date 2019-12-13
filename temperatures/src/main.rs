//  prints the temperature closest to 0 among input data.
//  If two numbers are equally close to zero, positive integer
//  has to be considered closest to zero (for instance, if the
//  temperatures are -5 and 5, then display 5).

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let inputs = String::from("1 -2 -8 4 5");

    for i in inputs.split_whitespace() {
        let t = parse_input!(i, i32);
        println!("{}", t);
    }
}
