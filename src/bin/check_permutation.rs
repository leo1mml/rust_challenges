use std::collections::hash_map::HashMap;

fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let char_occurrences_s1 = mount_char_occurrences(s1);
    let char_occurrences_s2 = mount_char_occurrences(s2);
    return char_occurrences_s2 == char_occurrences_s1;
}

fn mount_char_occurrences(s1: &str) -> HashMap<char, i32> {
    return s1.chars().fold(HashMap::new(), |mut h, value| {
        *h.entry(value).or_insert(0) += 1;
        return h;
    });
}

fn main() {
    println!("Hi folks! {}", is_permutation("aoeu", "aoeui"));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_different_sizes() {
        let s1 = "aoeu";
        let s2 = "aoeui";
        assert_eq!(is_permutation(s1, s2), false);
    }
    
    #[test]
    fn test_true_permutation() {
        let s = "aoeu";
        let s2 = "ueao";
        assert_eq!(is_permutation(s, s2), true);
    }

    #[test]
    fn test_false_permutation() {
        let s1 = "aoeu";
        let s2 = "snth";
        assert_eq!(is_permutation(s1, s2), false);
    }
}
