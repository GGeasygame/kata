use std::collections::HashSet;
use std::io::Write;
use crate::text_stats::text_stats;

pub fn print_result<W: Write>(writer: &mut W, amount_words: i32, amount_unique_words: i32, average_characters_of_words: f32, indexed_words: &Option<Vec<String>>, dictionary: &Option<HashSet<String>>) {
    write!(writer, "{}", get_output(amount_words, amount_unique_words, average_characters_of_words, indexed_words, dictionary)).expect("Could not write");
}

fn get_output(amount_words: i32, amount_unique_words: i32, average_characters_of_words: f32, indexed_words: &Option<Vec<String>>, dictionary: &Option<HashSet<String>>) -> String {
    let mut output = format!("Number of words: {}, unique: {}; average word length: {:.2} characters",
                             amount_words, amount_unique_words, average_characters_of_words);
    output += &get_indexes_output(indexed_words, dictionary);
    output
}

fn get_indexes_output(indexed_words: &Option<Vec<String>>, dictionary: &Option<HashSet<String>>) -> String {
    let mut output: String = "".to_string();
    if let Some(words) = indexed_words {
        match dictionary {
            Some(dict) => {
                let mut unknown_count = 0;
                let indexes: String = text_stats::for_unknown_words(words, dict, |index| {
                    unknown_count += 1;
                    return format!("{}*", index)
                }).iter().map(|word| format!("\n{}", word)).collect();
                output += &format!("\nIndex (unknown: {}):", unknown_count);
                output += &indexes
            }
            None => {
                output += "\nIndex:";
                let indexes: String = words.iter().map(|word| format!("\n{}", word)).collect();
                output += &indexes
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str;

    #[test]
    fn test_print_result_without_indexes_and_without_dictionary() {
        let mut writer = Vec::new();
        print_result(&mut writer, 10, 8, 3.56323, &None, &None);
        assert_eq!("Number of words: 10, unique: 8; average word length: 3.56 characters", str::from_utf8(writer.as_ref()).unwrap())
    }

    #[test]
    fn test_print_result_without_indexes_and_with_dictionary() {
        let mut writer = Vec::new();
        let dictionary = HashSet::from(["the".to_string(), "dicionary".to_string(), "should".to_string(), "not".to_string(), "make".to_string(), "a".to_string(), "difference".to_string()]);
        print_result(&mut writer, 10, 8, 3.56323, &None, &Some(dictionary));
        assert_eq!("Number of words: 10, unique: 8; average word length: 3.56 characters", str::from_utf8(writer.as_ref()).unwrap())
    }

    #[test]
    fn test_print_result_with_indexes_and_with_dictionary() {
        let mut writer = Vec::new();
        let indexes = Vec::from(["some".to_string(), "words".to_string(), "are".to_string(), "not".to_string(), "known".to_string()]);
        let dictionary = HashSet::from(["some".to_string(), "words".to_string(), "should".to_string(), "be".to_string(), "marked".to_string(), "as".to_string(), "unknown".to_string()]);
        print_result(&mut writer, 10, 8, 3.56323, &Some(indexes), &Some(dictionary));
        assert_eq!(
            r#"Number of words: 10, unique: 8; average word length: 3.56 characters
Index (unknown: 3):
some
words
are*
not*
known*"#,
                   str::from_utf8(writer.as_ref()).unwrap())
    }

    #[test]
    fn test_print_result_with_indexes_and_without_dictionary() {
        let mut writer = Vec::new();
        let indexes = Vec::from(["some".to_string(), "words".to_string(), "are".to_string(), "not".to_string(), "known".to_string()]);
        print_result(&mut writer, 10, 8, 3.56323, &Some(indexes), &None);
        assert_eq!(
            r#"Number of words: 10, unique: 8; average word length: 3.56 characters
Index:
some
words
are
not
known"#,
            str::from_utf8(writer.as_ref()).unwrap())
    }
}