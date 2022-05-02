//  prints the temperature closest to 0 among input data.
//  If two numbers are equally close to zero, positive integer
//  has to be considered closest to zero (for instance, if the
//  temperatures are -5 and 5, then display 5).

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn compare_0(x:i32, y:i32) -> i32 {
    match x + y {
        0 => x.abs(),
        _ => if x.abs() < y.abs() {x} else {y}
    }
}

fn main() {
    let inputs = String::from("1 -2 -8 4 5");
    let mut closest:i32 = 256;

    for i in inputs.split_whitespace() {
        let t = parse_input!(i, i32);
        closest = compare_0(closest, t);
    }

    println!("{:?}", closest);
}

#[test]
fn test_closest() {
    assert_eq!(compare_0(5, -5), 5);
    assert_eq!(compare_0(5, 3), 3);
    assert_eq!(compare_0(-5, -13), -5);
    assert_eq!(compare_0(7, -15), 7);
    assert_eq!(compare_0(7, -5), -5);
}
