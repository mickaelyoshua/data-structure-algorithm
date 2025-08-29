// https://www.hackerrank.com/challenges/pangrams/problem

use std::collections::HashSet;

fn pangrams_array(s: &str) -> String {
    let mut seen = [false; 26];
    let mut count = 0;

    for c in s.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            let index = c as usize - 'a' as usize;
            if !seen[index] {
                seen[index] = true;
                count += 1;
                if count == 26 {
                    return "pangram".to_string();
                }
            }
        }
    }
    "not pangram".to_string()
}

fn pangrams_hashset(s: &str) -> String {
    let unique_letters: HashSet<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic()) // all letters that are only alphabetics
        .collect();

    if unique_letters.len() == 26 { // if got all alphabetic letter
        "pangram".to_string()
    } else {
        "not pangram".to_string()
    }
}

fn main() {
    let s = "We promptly judged antique ivory buckles for the next prize";
    println!("{}", pangrams_array(s));

    let s_with_punctuation = "The quick brown fox jumps over the lazy dog.";
    println!("{}", pangrams_array(s_with_punctuation));

    let s = "We promptly judged antique ivory buckles for the next prize";
    println!("{}", pangrams_hashset(s));

    let s_with_punctuation = "The quick brown fox jumps over the lazy og.";
    println!("{}", pangrams_hashset(s_with_punctuation));
}
