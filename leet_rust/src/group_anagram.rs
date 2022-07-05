// 49. Group Anagrams 
use std::collections::HashMap;

pub fn group_anagram(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for s in strs.into_iter() {
        let mut key = [0; 26];

        for char in s.chars() {
            let idx  = char as u32 - 'a' as u32;
            key[idx as usize] += 1;
        }
        map.entry(key).or_insert(Vec::new()).push(s);
    }
    map.into_iter().map(|(_, v)| v ).collect()
}


pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for word in strs {
        let mut key = [0; 26];
        word.bytes().for_each(|byte| key[(byte - b'a') as usize] += 1);
        map.entry(key).or_insert(Vec::new()).push(word);
    }
    map.values().cloned().collect()
}


#[test]
fn some_test() {
    let _input  = vec!["eat","tea","tan","ate","nat","bat"];
    // Dont know how to do this yet:
    // let output = vec![["bat"],["nat","tan"],["ate","eat","tea"]];
}
