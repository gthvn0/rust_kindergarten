use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut val_to_idx: HashMap<i32, usize> = HashMap::new();
    let mut res: Vec<i32> = Vec::new();

    // First we keep value and its index in a hash table
    // Complexity: O(n)
    for (idx, val) in nums.iter().enumerate() {
        _ = val_to_idx.insert(*val, idx);
    }

    // Now we can check sum in a complexity of O(n)
    for (idx1, val1) in nums.iter().enumerate() {
        // We just need to check if we find a value that gives us the sum
        let val_expected = target - *val1;
        if let Some(idx2) = val_to_idx.get(&val_expected) {
            if *idx2 != idx1 {
                res.push(idx1 as i32);
                res.push(*idx2 as i32);
                return res;
            }
        }
    }

    // So complexity is 2*O(n) -> O(n)
    res
}

//Given an array of integers nums and an integer target, return indices of the
//two numbers such that they add up to target.
//You may assume that each input would have exactly one solution, and you may
//  not use the same element twice.
//
//You can return the answer in any order.
pub fn two_sum_brute(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // First solution using brute force
    // - check 1 with 2..nums.len
    // - check 2 with 3..nums.len
    // ...
    // - check nums.len-1 with nums.len..nums.len
    // Complexity is O(n2)
    let mut res: Vec<i32> = Vec::new();

    for idx1 in 0..nums.len() {
        for idx2 in idx1 + 1..nums.len() {
            let s = nums[idx1] + nums[idx2];
            if s == target {
                res.push(idx1 as i32);
                res.push(idx2 as i32);
                return res;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    fn example2() {
        let nums: Vec<i32> = vec![3, 2, 4];
        let target: i32 = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn example3() {
        let nums: Vec<i32> = vec![3, 3];
        let target: i32 = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
