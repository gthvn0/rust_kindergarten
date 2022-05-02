use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn compute_distance(x: f32, y:f32) -> f32 {


    return x + y;
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lon = input_line.trim().to_string();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let lat = input_line.trim().to_string();
    
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    for i in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let defib = input_line.trim_matches('\n').to_string();
    }


    println!("compute_distance {} {}: {}\n", 1.0, 2.0, compute_distance(1.0, 2.0));
    println!("answer");
}
