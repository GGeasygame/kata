use regex::Regex;
use std::collections::HashSet;

pub fn count_words(text: &str, stop_list: Vec<String>) -> i32 {
    let filter_set: HashSet<_> = stop_list.into_iter().collect();
    let re = Regex::new(r"[a-zA-Z]+").unwrap();

    re.find_iter(text)
        .filter(
            |regex_match| !filter_set.contains(regex_match.as_str())
        ).count() as i32
}

pub fn count_unique_words(text: &str, stop_list: Vec<String>) -> i32 {
    let filter_set: HashSet<_> = stop_list.into_iter().collect();
    let re = Regex::new(r"[a-zA-Z]+").unwrap();

    HashSet::<&str>::from_iter(
        re.find_iter(text)
            .filter(|regex_match| !filter_set.contains(regex_match.as_str()))
            .map(|regex_match| regex_match.as_str())
    ).len() as i32
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
        assert_eq!(4, count_words("marry had a little lamb", vec!["the", "a", "on", "off"].iter().map(|s| { s.to_string()}).collect()))
    }

    #[test]
    fn test_count_unique_words_with_duplicates() {
        assert_eq!(9, count_unique_words("Humpty-Dumpty sat on a wall. Humpty-Dumpty had a great fall.", vec![]))
    }

    #[test]
    fn test_count_unique_words_with_duplicates_and_stop_words() {
        assert_eq!(7, count_unique_words("Humpty-Dumpty sat on a wall. Humpty-Dumpty had a great fall.", vec!["the", "a", "on", "off"].iter().map(|s| { s.to_string()}).collect()))
    }
}