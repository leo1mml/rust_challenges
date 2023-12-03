use std::collections::HashMap;

fn process(input: &str) -> bool {
    let mut dictionary = HashMap::new();
    return input.chars().all(|c| {
        *dictionary.entry(c).or_insert(0) += 1;
        return dictionary[&c] <= 1;
    });
}

fn main() {
    let is_unique = process("");
    println!("Hello, world! {}", is_unique);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeating_initial_characters() {
        let word = "ccab";
        assert_eq!(process(word), false);
    }

    #[test]
    fn test_repeating_edge_characters() {
        let word = "ccabc";
        assert_eq!(process(word), false);
    }

    #[test]
    fn test_non_repeating() {
        let word = "cab";
        assert_eq!(process(word), true);
    }
}
