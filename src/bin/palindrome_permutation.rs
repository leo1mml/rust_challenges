use std::collections::HashMap;

fn is_palindrome_permutation(value: &str) -> bool {
    if value.len() == 1 {
        return true;
    }
    let value = value.to_lowercase().replace(" ", "");
    let mut map: HashMap<char, i32> = HashMap::new();
    for char in value.chars() {
        *map.entry(char).or_insert(0) += 1;
    }
    let mut odd_occurrences = 0;
    map.into_iter().for_each(|(_, value)| {
        if value % 2 != 0 {
            odd_occurrences += 1;
        }
    });
    if odd_occurrences > 1 {
        return false;
    }
    return true;
}

fn main() {
    println!("Hello world!!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation_of_palindrome_no_palindrome() {
        let value2 = "tutate";
        assert!(!is_palindrome_permutation(value2));
    }

    #[test]
    fn test_is_permutation_of_palindrome_is_palindrome() {
        let value2 = "Tact Coa";
        assert!(is_palindrome_permutation(value2));
    }
}
