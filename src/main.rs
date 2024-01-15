use std::collections::HashMap;

// program that takes in two strings, and checks if they are anagram or not.
// Inputs: str1, str2
// process: check anagram
// output: "str1" and "str2" {are | are not} anagrams

fn is_anagram(str1: &str, str2: &str) -> bool {
    // if not the same length, return false
    if str1.len() != str2.len() {
        return false;
    }

    let mut str1_frequency: HashMap<char, i32> = HashMap::new();
    for c in str1.chars() {
        *str1_frequency.entry(c).or_insert(1) += 1;
    }

    let mut str2_frequency: HashMap<char, i32> = HashMap::new();
    for c in str2.chars() {
        *str2_frequency.entry(c).or_insert(1) += 1;
    }

    for (key, value) in &str1_frequency {
        match str2_frequency.get(key) {
            Some(str2_value) => {
                if value - str2_value != 0 {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(is_anagram("note", "tone"), true);
        assert_eq!(is_anagram("listen", "silent"), true);
        assert_eq!(is_anagram("a", "as"), false);
        assert_eq!(is_anagram("car", "rar"), false);
    }
}

fn main() {
    println!("Hello, world!");
}
