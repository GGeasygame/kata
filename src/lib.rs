use std::collections::HashSet;
use regex::{Match, Regex};


const WORD_REGEX: &str = r"[a-zA-Z-]+";

pub fn count_words(text: &String, stop_list: &HashSet<String>) -> i32 {
    let regex = Regex::new(WORD_REGEX).unwrap();
    find_matches(text, stop_list, &regex).count() as i32
}

pub fn count_unique_words(text: &String, stop_list: &HashSet<String>) -> i32 {
    let regex = Regex::new(WORD_REGEX).unwrap();
    HashSet::<&str>::from_iter(
        find_matches(text, stop_list, &regex)
            .map(|regex_match| regex_match.as_str())
    ).len() as i32
}

pub fn calculate_average_characters_of_words(text: &String, stop_list: &HashSet<String>) -> f32 {
    let regex = Regex::new(WORD_REGEX).unwrap();
    let matches: Vec<_> = find_matches(text, stop_list, &regex).collect();
    match matches.clone().into_iter().map(|regex_match| regex_match.len()).reduce(|acc, curr| acc + curr) {
        Some(value) => (value as f32) / (matches.len() as f32),
        None => 0f32
    }
}

pub fn get_indexed_words(text: &String, stop_list: &HashSet<String>, dictionary: &Option<HashSet<String>>) -> Vec<String> {
    let regex = Regex::new(WORD_REGEX).unwrap();
    match dictionary {
        Some(value) => {
            let mut matches = Vec::from_iter(find_matches(text, stop_list, &regex)
                .map(|regex_match| {
                    let word = regex_match.as_str().to_string();
                    if !value.contains(&word) {
                        return format!("{}*", word)
                    }
                    word
                }));
            matches.sort();
            matches
        }
        None => {
            let mut matches = Vec::from_iter(find_matches(text, stop_list, &regex)
                .map(|regex_match| {
                    let word = regex_match.as_str().to_string();
                    word
                }));
            matches.sort();
            matches
        }
    }
}

fn find_matches<'a>(text: &'a String, stop_list: &'a HashSet<String>, regex: &'a Regex) -> impl Iterator<Item=Match<'a>> + 'a {
    regex.find_iter(text)
        .filter(
            move |regex_match| !stop_list.contains(regex_match.as_str())
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words_little_lamb_poem() {
        assert_eq!(5, count_words(&"marry had a little lamb".to_string(), &HashSet::new()))
    }

    #[test]
    fn test_count_words_text_with_two_whitespaces() {
        assert_eq!(5, count_words(&"marry had a little  lamb".to_string(), &HashSet::new()))
    }

    #[test]
    fn test_count_words_empty_string() {
        assert_eq!(0, count_words(&"".to_string(), &HashSet::new()))
    }

    #[test]
    fn test_count_words_with_stopwords() {
        assert_eq!(4, count_words(&"marry had a little lamb".to_string(), &["the", "a", "on", "off"].iter().map(|s| { s.to_string() }).collect()))
    }

    #[test]
    fn test_count_unique_words_with_duplicates() {
        assert_eq!(11, count_unique_words(&"there are duplicates in this text, try to find them in the text!".to_string(), &HashSet::new()))
    }

    #[test]
    fn test_count_unique_words_with_duplicates_and_stop_words() {
        assert_eq!(10, count_unique_words(&"there are duplicates in this text, try to find them in the text!".to_string(), &["the", "a", "on", "off"].iter().map(|s| { s.to_string() }).collect()))
    }

    #[test]
    fn test_count_words_with_hyphens() {
        assert_eq!(10, count_words(&"Humpty-Dumpty sat on a wall. Humpty-Dumpty had a great fall.".to_string(), &HashSet::new()))
    }

    #[test]
    fn test_count_unique_words_with_hyphens() {
        assert_eq!(8, count_unique_words(&"Humpty-Dumpty sat on a wall. Humpty-Dumpty had a great fall.".to_string(), &HashSet::new()))
    }

    #[test]
    fn test_calculate_average_characters_of_words() {
        assert_eq!(3.8, calculate_average_characters_of_words(&"marry had a little lamb".to_string(), &HashSet::new()))
    }

    #[test]
    fn test_get_indexed_words() {
        assert_eq!(vec!["a", "had", "lamb", "little", "marry"], get_indexed_words(&"marry had a little lamb".to_string(), &HashSet::new(), &None))
    }

    #[test]
    fn test_get_indexed_words_with_dictionary() {
        assert_eq!(vec!["a", "had", "lamb*", "little", "marry*"], get_indexed_words(&"marry had a little lamb".to_string(), &HashSet::new(), &Some(["a", "had", "little"].iter().map(|s| { s.to_string() }).collect())))
    }
}