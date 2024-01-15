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

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // print "Enter two strings and I'll tell you if they are anagrams."
    println!("Enter two strings and I'll tell you if they are anagrams.");
    // str1 input "Enter the first string: "
    let str1: String = get_input("Enter the first string: ");
    // str2 input "Enter the second string: "
    let str2: String = get_input("Enter the second string: ");
    // check is_anagram
    let status: &str = match is_anagram(&str1, &str2) {
        true => "are",
        false => "are not"
    }
    // print "{str1}" and "{str2}" {are | are not} anagrams.
    println!("\"{}\" and \"{}\" {} anagrams.", str1, str2, status);
}
