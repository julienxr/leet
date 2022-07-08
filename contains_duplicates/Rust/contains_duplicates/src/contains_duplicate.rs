use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut unique_vals = HashSet::new();
    for num in &nums {
        if unique_vals.contains(&num) {
            return true;
        }
        unique_vals.insert(num);
    }
    false
}
