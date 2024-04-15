// Title: Longest Substring Without Repeating Characters
// Problem: Given a string s, find the length of the longest substring without repeating characters.

use std::collections::HashSet;
use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut visited = HashSet::new();
        let mut res = 0;
        let mut start = 0;

        for (end, ch) in s.chars().enumerate() {
            while visited.contains(&ch) {
                visited.remove(&s.chars().nth(start).unwrap());
                start += 1;
            }
            visited.insert(ch);
            res = max(res, end - start + 1);
        }
        res as i32
    }
}

