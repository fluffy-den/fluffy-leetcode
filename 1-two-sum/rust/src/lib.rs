pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if (nums[i] + nums[j]) == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let res = two_sum([1, 2].to_vec(), 3);
        assert_eq!(res, [0, 1]);
    }
}
