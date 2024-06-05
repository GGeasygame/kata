use regex::Regex;

pub fn count_words(text: &str) -> i32 {
    let re = Regex::new(r"[a-zA-Z]+").unwrap();
    re.find(text).unwrap().len() as i32

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_little_lamb_poem() {
        assert_eq!(5, count_words("marry had a little lamb"))
    }

    #[test]
    fn test_count_words_text_with_two_whitespaces() {
        assert_eq!(5, count_words("marry had a little  lamb"))
    }
}