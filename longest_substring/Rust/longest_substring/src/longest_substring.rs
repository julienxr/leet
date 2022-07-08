use std::collections::HashMap;
use std::cmp::max;

pub fn length_of_longest_substring_map(s: String) -> i32 {
    let mut start: usize = 0;
    let mut longest = 0;
    let mut map: HashMap<char, usize> = HashMap::new();


    for (idx, ch) in s.char_indices() {
        map.entry(ch)
            .and_modify(|old_idx| {
                if *old_idx >= start {
                    // there is a repitition
                    longest = max(longest, idx - start);
                    start = *old_idx + 1;
                }
                *old_idx = idx;
            }).or_insert(idx);
    }
    max(longest, s.len() - start) as i32
}

pub fn length_of_longest_substring_match(s: String) -> i32 {
    let mut longest = 0_i32;
    let mut seen = HashMap::new();
    let mut start = 0_usize;

    for (idx, ch) in s.char_indices() {
        match seen.insert(ch, idx) {
            Some(existing_idx) if existing_idx >= start => {
                longest = max(longest, start as i32 - existing_idx as i32);
                start = existing_idx + 1;
            }
            _ => {
                longest = max(longest, idx as i32 - start as i32 + 1);
            }
        }
    }
    longest
}

pub fn length_of_longest_substring_iter(s: String) -> i32 {
    let mut current: Vec<char> = Vec::new();
    let mut max_length = 0;

    s.chars().for_each(|c| {
        if let Some(i) = current.iter().position(|&x| x == c) {
            current.drain(0..i+1);
        }
        current.push(c);
        max_length = max(current.len(), max_length);
    });
    max_length as i32
}


#[test]
fn some_test() {
    let data = vec![("abcabcbb",3),
                    ("bbbbb",1),
                    ("pwwkew",3),
                    ("a", 1),
                    ("AAACDEFGH", 7),
                    ];
    for (s,l) in data {
        let length = length_of_longest_substring_map(s.to_string());
        assert_eq!(l, length, "for {}", s);
    }
}
