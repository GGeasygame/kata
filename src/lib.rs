use regex::Regex;

pub fn count_words(text: &str, vec1: Vec<&str>) -> i32 {
    let re = Regex::new(r"[a-zA-Z]+").unwrap();
    match re.find(text) {
        Some(value) => value.len() as i32,
        None => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_little_lamb_poem() {
        assert_eq!(5, count_words("marry had a little lamb", ))
    }

    #[test]
    fn test_count_words_text_with_two_whitespaces() {
        assert_eq!(5, count_words("marry had a little  lamb", ))
    }

    #[test]
    fn test_count_words_empty_string() {
        assert_eq!(0, count_words("", ))
    }

    #[test]
    fn test_count_words_with_stopwords() {
        assert_eq!(4, count_words("marry had a little lamb", vec!["the", "a", "on", "off"]))
    }
}