mod convert_base {
    pub fn number_to_base_string(number: i32, base: i32) -> String {
        if number < base {
            return format!("{}", convert_decimal_to_hex_digit(number));
        }
        let remainder = number % base;
        let remainder_string = convert_decimal_to_hex_digit(remainder);
        let reduced_number = (number - remainder)/ base; 
        return format!("{}{}", number_to_base_string(reduced_number, base), remainder_string);
    }

    fn convert_decimal_to_hex_digit(decimal: i32) -> String {
        let digit = match decimal {
            0..=9 => format!("{}", decimal),
            10 => "A".to_string(),
            11 => "B".to_string(),
            12 => "C".to_string(),
            13 => "D".to_string(),
            14 => "E".to_string(),
            15 => "F".to_string(),
            i32::MIN..=-1_i32 | 16_i32..=i32::MAX => todo!()

        };
        return digit.to_string();
    }

    pub fn base_string_to_value(digits: &str, base: i32) -> i32 {
        if digits.is_empty() {
            return 0;
        }
        let last_digit_value = digits.chars().last()
            .map(|n| n.to_digit(base as u32).unwrap_or(0) as i32);
        let mut output = last_digit_value.unwrap_or(0);
        let remainder_string = &digits[..digits.len() - 1];
        output += base * base_string_to_value(remainder_string, base);
        return output;
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::convert_base::*;

    #[test]
    fn convert_binary_to_decimal() {
        assert_eq!("11", number_to_base_string(3, 2));
        assert_eq!("10", number_to_base_string(2, 2));
        assert_eq!("10", number_to_base_string(10, 10));
        assert_eq!("B", number_to_base_string(11, 16));
    }

    #[test]
    fn convert_string_to_decimal() {
        assert_eq!(10, base_string_to_value("10", 10));
        assert_eq!(base_string_to_value("123", 10), 123);
        assert_eq!(base_string_to_value("A1", 16), 161);
        assert_eq!(base_string_to_value("101", 2), 5);
    }
}
