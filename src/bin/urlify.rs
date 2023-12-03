fn urlify(text: &str) -> String {
    return text.to_string().replace(' ', "%20");
}

fn main() {
    let text = urlify("Hello world!!");
    println!("{}", text);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_replace_space_with_percent() {
        let str1 = "www.test.rust.com/aoeu aooeeo";
        let str2 = "www.test.rust.com/aoeu%20aooeeo";
        assert_eq!(urlify(str1), str2) 
    }
}
