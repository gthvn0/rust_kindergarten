// i32 to vec<i32>: 12 -> [1, 2]
fn itov(x: i32) -> Vec<i32> {
    let mut v = Vec::new();
    let mut i = x;

    while i >= 10 {
        v.push(i % 10);
        i /= 10;
    }

    v.push(i);
    v.into_iter().rev().collect()
}

fn at_pos(value: i32, pos: usize) -> i32 {
    let mut res = value;
    for _ in 0..pos {
        res /= 10;
    }
    res % 10
}

// 1 3 1 -> len = 3,
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    if x < 10 {
        return true;
    }

    let v = itov(x);
    println!("v: {:?}", v);
    println!("len/2: {}", v.len() / 2);
    for idx in 0..v.len() / 2 {
        println!("Comparing {} and {}", v[idx], v[v.len() - 1 - idx]);
        if v[idx] != v[v.len() - 1 - idx] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_pos_123_0() {
        assert_eq!(at_pos(123, 0), 3);
    }

    #[test]
    fn at_pos_123_1() {
        assert_eq!(at_pos(123, 1), 2);
    }

    #[test]
    fn at_pos_123_2() {
        assert_eq!(at_pos(123, 2), 1);
    }

    #[test]
    fn at_pos_123_3() {
        assert_eq!(at_pos(123, 3), 0);
    }

    #[test]
    fn itov_exemple123() {
        assert_eq!(itov(123), vec![1, 2, 3])
    }

    #[test]
    fn itov_exemple10() {
        assert_eq!(itov(10), vec![1, 0])
    }

    #[test]
    fn itov_exemple0() {
        assert_eq!(itov(0), vec![0])
    }

    #[test]
    fn itov_exemple9() {
        assert_eq!(itov(9), vec![9])
    }

    #[test]
    fn example10() {
        assert_eq!(is_palindrome(10), false);
    }

    #[test]
    fn example121() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    fn example12345_43211() {
        assert_eq!(is_palindrome(12345_43211), false);
    }

    #[test]
    fn example1234_5_4321() {
        assert_eq!(is_palindrome(1234_5_4321), true);
    }

    #[test]
    fn example1233_5_4321() {
        assert_eq!(is_palindrome(1233_5_4321), false);
    }

    #[test]
    fn example1234_4321() {
        assert_eq!(is_palindrome(1234_4321), true);
    }
}
