/// Adds comma every 3 digits
pub fn format_number(n: f64) -> String {
    let n = n.to_string();
    let mut result = String::new();
    let mut count = 0;

    for c in n.chars().rev() {
        count += 1;
        result.push(c);

        if count % 3 == 0 && count != n.len() {
            result.push(',');
        }
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1000.0), "1,000");
        assert_eq!(format_number(1000000.0), "1,000,000");
        assert_eq!(format_number(1000000000.0), "1,000,000,000");
        assert_eq!(format_number(1000000000000.0), "1,000,000,000,000");
        assert_eq!(format_number(999999.0), "999,999");
    }
}
