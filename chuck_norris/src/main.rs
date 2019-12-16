//
// Binary with 0 and 1 is good, but binary with only 0, or almost,
// is even better!
//
// Two consecutive blocks are used to produce a series of same value
// bits (only 1 or 0 values):
//   - First block: it is always 0 or 00. If it is 0, then the series
//     contains 1, if not, it contains 0
//   - Second block: the number of 0 in this block is the number of
//     bits in the series

fn to_binary(x: usize) -> Vec<u8> {

    if x == 0 {
        return vec![0];
    }

    const DIVISOR:usize = 2;
    let mut v = Vec::new();
    let mut quotient = x;
    let mut reminder;

    while quotient != 0 {
        let x = quotient;

        quotient = x / DIVISOR;
        reminder = x % DIVISOR;
        v.push(reminder as u8);
    }

    v.reverse();
    v
}

fn main() {
    println!("0 -> {:?}", to_binary(0));
    println!("1 -> {:?}", to_binary(1));
    println!("2 -> {:?}", to_binary(2));
    println!("3 -> {:?}", to_binary(3));

    let c_ascii = 'C' as usize;
    println!("C: {} ({:b}) -> {:?}", c_ascii, c_ascii, to_binary(c_ascii));

    let c_ascii = 'c' as usize;
    println!("C: {} ({:b}) -> {:?}", c_ascii, c_ascii, to_binary(c_ascii));
}
