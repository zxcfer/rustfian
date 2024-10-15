// function that takes a vector of integers and a target sum and returns a vector of two integers that add up to the target sum
pub fn sum_target(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();
    for i in 0..nums.len() {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                result.push(nums[i]);
                result.push(nums[j]);
                return result;
            }
        }
    }
    result
}