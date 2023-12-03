fn number_to_base_string(number: i32, base: i32) -> String {
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

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_binary_to_decimal() {
        assert_eq!("11", number_to_base_string(3, 2));
        assert_eq!("10", number_to_base_string(2, 2));
        assert_eq!("10", number_to_base_string(10, 10));
        assert_eq!("B", number_to_base_string(11, 16));
    }
}
