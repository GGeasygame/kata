use regex::Regex;
use std::collections::HashSet;

pub fn count_words(text: &str, stop_list: Vec<&str>) -> i32 {
    let filter_set: HashSet<_> = stop_list.into_iter().collect();
    let re = Regex::new(r"[a-zA-Z]+").unwrap();

    re.find_iter(text)
        .filter(
            |regex_match| !filter_set.contains(regex_match.as_str())
        ).count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_little_lamb_poem() {
        assert_eq!(5, count_words("marry had a little lamb", vec![]))
    }

    #[test]
    fn test_count_words_text_with_two_whitespaces() {
        assert_eq!(5, count_words("marry had a little  lamb", vec![]))
    }

    #[test]
    fn test_count_words_empty_string() {
        assert_eq!(0, count_words("", vec![]))
    }

    #[test]
    fn test_count_words_with_stopwords() {
        assert_eq!(4, count_words("marry had a little lamb", vec!["the", "a", "on", "off"]))
    }
}