fn add_binary_strings(b1: &str, b2: &str) -> String {
    let len1 = b1.len();
    let len2 = b2.len();
    let max_len = len1.max(len2);

    // Pad the binary strings with leading zeros
    // 000
    let b1 = "0".repeat(max_len - len1) + b1;
    // 101
    let b2 = "0".repeat(max_len - len2) + b2;
    let mut output = "".to_string();
    let mut carry_digit = '0';

    b1.chars()
        .rev()
        .zip(b2.chars().rev())
        .for_each(|(c1, c2)| {
            if c1 == c2 {
                let write_digit = if c1 == '0' && carry_digit == '1' {'1'} else {'0'};
                output.insert(0, 
                              write_digit);
                carry_digit = if c1 == '1' {'1'} else {'0'}
            } else {
                let write_digit = if carry_digit == '1' {'0'} else {'1'};
                output.insert(0, write_digit);
                carry_digit = if carry_digit == '1' {'1'} else {'0'}
            }
        });
    if carry_digit == '1' {
        output.insert(0, '1')
    }
    return output;
}

fn main() {
    let text = add_binary_strings("1101", "1001");
    print!("{}", text);
}

#[cfg(test)]
mod test {
    use crate::add_binary_strings;

    #[test]
    fn add_binaries() {
        assert_eq!(add_binary_strings("01", "01"), "10");
        let b1 = "0101";
        let b2 = "1101";
        let result = add_binary_strings(b1, b2);
        assert_eq!(result, "10010");

        // Test case 2: Different length strings
        let b3 = "110";
        let b4 = "011";
        let result2 = add_binary_strings(b3, b4);
        assert_eq!(result2, "1001");

        // Test case 3: Empty strings
        let b5 = "";
        let b6 = "101";
        let result3 = add_binary_strings(b5, b6);
        assert_eq!(result3, "101");

        // Test case 4: Strings with leading zeros
        let b7 = "00101";
        let b8 = "1101";
        let result4 = add_binary_strings(b7, b8);
        assert_eq!(result4, "10010");

        // Test case 5: Large numbers
        let b9 = "1111111111111111111111111111111";
        let b10 = "1";
        let result5 = add_binary_strings(b9, b10);
        assert_eq!(result5, "10000000000000000000000000000000");
    }
}
