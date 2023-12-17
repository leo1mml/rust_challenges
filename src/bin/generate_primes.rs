
fn main() {
   println!("{:?}", generate_primes(50)) 
}

fn generate_primes(limit: i32) -> Vec<i32> {
    if limit <= 3 {
        return (2..=limit).collect();
    }
    let max_multiplier = (limit as f32).sqrt() as i32;
    let mut numbers_to_check: Vec<i32> = (2..=max_multiplier).collect();
    numbers_to_check.reverse();
    let mut primes: Vec<i32> = (2..=limit).collect();
    while let Some(divisor) = numbers_to_check.pop() {
        primes = primes
            .iter()
            .map(|v| *v)
            .filter(|value| {
                *value <= divisor || value % divisor != 0
            })
        .collect();
    }
    return primes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_primes() {
        assert_eq!(generate_primes(10), [2 , 3, 5, 7]);
        assert_eq!(generate_primes(4), [2 , 3]);
        assert_eq!(generate_primes(2), [2]);
    }
}
