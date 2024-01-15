// program that takes in two strings, and checks if they are anagram or not.
// Inputs: str1, str2
// process: check anagram
// output: "str1" and "str2" {are | are not} anagrams

def is_anagram(str1: String, str2: String) -> bool {
    //
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(is_anagram("note".to_string(), "tone".to_string()), true);
        assert_eq!(is_anagram("listen".to_string(), "silent".to_string()), true);
        assert_eq!(is_anagram("a".to_string(), "as".to_string()), false);
        assert_eq!(is_anagram("car".to_string(), "rar".to_string()), false);
    }
}

fn main() {
    println!("Hello, world!");
}
