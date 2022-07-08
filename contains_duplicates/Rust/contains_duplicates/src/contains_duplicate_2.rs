use std::collections::HashMap;
/* Problem #219. Contains Duplicate 2
 *
 * Given an integer array "nums" and an integer "k", return true if there are
 * two distinct indices "i" and "j" in the array such that "nums[i] == nums[j]"
 * and "abs(i-j) <= k. In other words find duplicate values with in 'k' distance
 * of 'j' from 'i' "
 */

// #1 Naive approach (Linear Search)

pub fn contains_nearby_duplicate_linear(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let len = nums.len();
    for i in 0..len {
        for n in &nums[(i + 1) ..len.min(i + 1 + k)]{
            if nums[i] - n == 0 {
                return true;
            }
        }
    }
    false
}


// #2 HashMap
pub fn contains_nearby_duplicate_hashmap(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut map = HashMap::new();
    for i in 0..nums.len() {
        if map.contains_key(&nums[i]) && i <= map[&nums[i]] + k {
            return true;
        }
        else {
            map.insert(nums[i], i);
        }
    }
    false
}


// #3 Hash Table
pub fn contains_nearby_duplicate_sort(nums: Vec<i32>, k: i32) -> bool {
    let k = k as usize;
    let mut v = Vec::new();
    for i in 0..nums.len() {
        v.push((nums[i], i));
    }
    v.sort_unstable();
    for i in 1..v.len() {
        if v[i - 1].0 == v[i].0 && v[i].1 <= v[i - 1].1 + k {
            return true;
        }
    }
    false
}

// #4 HashMap (fastest)
pub fn contains_nearby_duplicate_hashmap2(nums: Vec<i32>, k: i32) -> bool {
    if nums.len() < 2 {
                return false
            }
            // put num as key in hashmap with most recent index of that num in nums as value
            let mut map = HashMap::new();
            for i in 0..nums.len() {
                match map.insert(nums[i], i) { 
                    //HashMap::insert returns an Option<val> of previous value for given key
                    // some solutions are casting k as usize once rather than casting the indices
                    // as i32, however this could overflow as there is no guarantee that k will
                    // fit as usize, so even if there is some performance hit doing it this way
                    // it is preferable.
                    Some(x) if (i-x) as i32 <= k => return true,
                    _ => continue
                }
            }
            false
}
