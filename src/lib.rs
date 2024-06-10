use regex::Regex;
use std::collections::HashSet;

pub fn count_words(text: &str, stop_list: Vec<String>) -> i32 {
    let filter_set: HashSet<_> = stop_list.into_iter().collect();
    let re = Regex::new(r"[a-zA-Z-]+").unwrap();

    re.find_iter(text)
        .filter(
            |regex_match| !filter_set.contains(regex_match.as_str())
        ).count() as i32
}

pub fn count_unique_words(text: &str, stop_list: Vec<String>) -> i32 {
    let filter_set: HashSet<_> = stop_list.into_iter().collect();
    let re = Regex::new(r"[a-zA-Z-]+").unwrap();

    HashSet::<&str>::from_iter(
        re.find_iter(text)
            .filter(|regex_match| !filter_set.contains(regex_match.as_str()))
            .map(|regex_match| regex_match.as_str())
    ).len() as i32
}

pub fn calculate_average_characters_of_words(text: &str, stop_list: Vec<String>) -> f32 {
    let filter_set: HashSet<_> = stop_list.into_iter().collect();
    let re = Regex::new(r"[a-zA-Z-]+").unwrap();

    let matches: Vec<_> = re.find_iter(text)
        .filter(
            |regex_match| !filter_set.contains(regex_match.as_str())
        ).collect();
    match matches.clone().into_iter().map(|regex_match| regex_match.len()).reduce(|acc, curr| acc + curr) {
        Some(value) => (value as f32) / (matches.len() as f32),
        None => 0f32
    }
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
        assert_eq!(11, count_unique_words("there are duplicates in this text, try to find them in the text!", vec![]))
    }

    #[test]
    fn test_count_unique_words_with_duplicates_and_stop_words() {
        assert_eq!(10, count_unique_words("there are duplicates in this text, try to find them in the text!", vec!["the", "a", "on", "off"].iter().map(|s| { s.to_string()}).collect()))
    }

    #[test]
    fn test_count_words_with_hyphens() {
        assert_eq!(10, count_words("Humpty-Dumpty sat on a wall. Humpty-Dumpty had a great fall.", vec![]))
    }

    #[test]
    fn test_count_unique_words_with_hyphens() {
        assert_eq!(8, count_unique_words("Humpty-Dumpty sat on a wall. Humpty-Dumpty had a great fall.", vec![]))
    }

    #[test]
    fn test_calculate_average_characters_of_words() {
        assert_eq!(3.8, calculate_average_characters_of_words("marry had a little lamb", vec![]))
    }
}