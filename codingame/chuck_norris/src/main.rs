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

// Hello -> [72, 101, 101, 11]
fn decompose(s: &str) -> Vec<u8> {
    let mut res = Vec::new();
    for c in s.chars() {
       res.push(c as u8);
    }
    res
}

// binary is returning the ASCII 7-bits representation
// 12 -> [0, 0, 0, 1, 1, 0, 0]
fn u8_to_binary(x: u8) -> Vec<u8> {
    const BITS: usize = 7;
    const MAXVAL: u8 = 127;

    if x > MAXVAL {
        panic!("{} is not allowed on 7-bits", x);
    }

    if x == 0 {
        return vec![0, 0, 0, 0, 0, 0, 0];
    }

    let mut v = Vec::new();
    let mut val = x;

    while val != 0 {
        v.push((val & 0x1) as u8);
        val >>= 1;
    }

    // Padding
    for _ in v.len()..BITS {
        v.push(0 as u8);
    }

    v.reverse();
    v
}

// [12, 6] -> [1, 1, 0, 0, 1, 1, 0]
fn vec_to_binary(v: &[u8]) -> Vec<u8> {
    let mut res = Vec::new();
    for elmt in v {
        res.extend(u8_to_binary(*elmt));
    }
    res
}

// [1, 1, 0, 0, 0, 1, 1, 0, 1] => [(2, 1), (3, 0), (2, 1), (1, 0), (1, 1)]
fn transform_vec(v: &[u8]) -> Vec<(u8, u8)> {
    let mut res = Vec::new();
    let mut current_val = v[0];
    let mut current_num = 0;

    for val in v.iter() {
        if *val == current_val {
            current_num += 1;
        } else  {
            res.push((current_num, current_val));
            current_val = *val;
            current_num = 1;
        }
    }
    res.push((current_num, current_val));
    res
}

// (2, 1) -> 0 00
// (3, 0) -> 00 000
// (1, 0) -> 00 0
// ...
fn speak_chuck(nb: u8, val: u8) -> String {
    let mut chuck = if val == 0 {String::from("00 ")} else {String::from("0 ")};
    for _ in 0..nb {
        chuck.push('0');
    }

    chuck
}

fn chuck_translator(s: &str) -> String {

    let mut chuck = String::new();
    let vect = decompose(s);
    let bin = vec_to_binary(&vect);
    let trans = transform_vec(&bin);

    for (n, v) in trans {
        chuck.push_str(&speak_chuck(n, v));
        chuck.push(' ');
    }

    // just pop the last spurious space
    chuck.pop().unwrap();
    chuck
}

fn main() {

    println!("{}", chuck_translator("Hello"));
    println!("{}", chuck_translator("C"));
    println!("{}", chuck_translator("CC"));
}

#[test]
fn test_chuck_translator() {
    let hello = String::from("0 0 00 00 0 0 00 000 0 00 00 00 0 0 00 0 0 000 00 0 0 00 00 00 0 00 00 0 0 00 00 00 0 00 00 0 0 0000");
    let c = String::from("0 0 00 0000 0 00");
    let cc = String::from("0 0 00 0000 0 000 00 0000 0 00");
    let percentage = String::from("00 0 0 0 00 00 0 0 00 0 0 0");

    assert_eq!(chuck_translator("Hello"), hello);
    assert_eq!(chuck_translator("C"), c);
    assert_eq!(chuck_translator("CC"), cc);
    assert_eq!(chuck_translator("%"), percentage);
}

#[test]
fn test_vec_to_binary() {
    assert_eq!(vec_to_binary(&vec![12, 6]), vec![0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0]);
}

#[test]
fn test_decompose() {
    assert_eq!(decompose("Hello"), vec![72, 101, 108, 108, 111]);
}

#[test]
fn test_u8_to_binary() {
    assert_eq!(u8_to_binary(0),   vec![0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(u8_to_binary(1),   vec![0, 0, 0, 0, 0, 0, 1]);
    assert_eq!(u8_to_binary(6),   vec![0, 0, 0, 0, 1, 1, 0]);
    assert_eq!(u8_to_binary(12),  vec![0, 0, 0, 1, 1, 0, 0]);
    assert_eq!(u8_to_binary(41),  vec![0, 1, 0, 1, 0, 0, 1]);
    assert_eq!(u8_to_binary(118), vec![1, 1, 1, 0, 1, 1, 0]);
    assert_eq!(u8_to_binary(127), vec![1, 1, 1, 1, 1, 1, 1]);
}

#[test]
fn test_transform_vec() {
    assert_eq!(transform_vec(&[1, 1, 0, 0, 0, 1, 1, 0, 1]), vec![(2, 1), (3, 0), (2, 1), (1, 0), (1, 1)]);
    assert_eq!(transform_vec(&[1, 0, 0, 0, 0, 1]), vec![(1, 1), (4, 0), (1, 1)]);
    assert_eq!(transform_vec(&[1]), vec![(1, 1)]);
    assert_eq!(transform_vec(&[0]), vec![(1, 0)]);
}

#[test]
fn test_speak_chuck() {
    assert_eq!(speak_chuck(3, 0), "00 000");
    assert_eq!(speak_chuck(1, 0), "00 0");
    assert_eq!(speak_chuck(5, 1), "0 00000");
}
