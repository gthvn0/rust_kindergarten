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

// 12 -> [1, 1, 0, 0]
fn to_binary(x: usize) -> Vec<u8> {
    if x == 0 {
        return vec![0];
    }

    let mut v = Vec::new();
    let mut val = x;

    while val != 0 {
        v.push((val & 0x1) as u8);
        val >>= 1;
    }

    v.reverse();
    v
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

fn main() {

    let c_ascii = 'C' as usize;
    let v1 = to_binary(c_ascii);
    print!("C: {} ({:b}) -> {:?} -> ", c_ascii, c_ascii, v1);

    let trans =  transform_vec(&v1);
    print!("{:?} -> ", trans);

    for (n, v) in trans {
        print!("{} ", speak_chuck(n, v));
    }

    println!("\n");
}

#[test]
fn test_to_binary() {
    assert_eq!(to_binary(0), vec![0]);
    assert_eq!(to_binary(1), vec![1]);
    assert_eq!(to_binary(12), vec![1, 1, 0, 0]);
    assert_eq!(to_binary(41), vec![1, 0, 1, 0, 0, 1]);
    assert_eq!(to_binary(118), vec![1, 1, 1, 0, 1, 1, 0]);
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
