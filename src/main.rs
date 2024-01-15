use std::collections::HashMap;

// program that takes in two strings, and checks if they are anagram or not.
// Inputs: str1, str2
// process: check anagram
// output: "str1" and "str2" {are | are not} anagrams

fn get_char_frequency(s: &str) -> HashMap<char, i32> {
    let mut frequency: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *frequency.entry(c).or_insert(0) += 1;
    }
    frequency
}

fn is_anagram(str1: &str, str2: &str) -> bool {
    // if not the same length, return false
    if str1.len() != str2.len() {
        return false;
    }

    let str1_frequency: HashMap<char, i32> = get_char_frequency(str1);
    let str2_frequency: HashMap<char, i32> = get_char_frequency(str2);

    for (key, value) in &str1_frequency {
        match str2_frequency.get(key) {
            Some(str2_value) => {
                if !value.eq(str2_value) {
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
