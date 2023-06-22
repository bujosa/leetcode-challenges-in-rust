#![allow(dead_code)]
pub fn is_palindrome(s: String) -> bool {
    let mut chars = s.chars().filter(|c| c.is_ascii_alphanumeric());
    while let (Some(left), Some(right)) = (chars.next(), chars.next_back()) {
        if !left.eq_ignore_ascii_case(&right) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_palindrome() {
        assert_eq!(is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(" ".to_string()), true);

        assert_eq!(is_palindrome("".to_string()), true);

        assert_eq!(
            is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }
}
