use std::fs;
use std::io::{BufRead, stdout, Write};

pub fn get_input_text<R: BufRead>(text_file_path: &Option<&String>, reader: R) -> String {
    match text_file_path {
        Some(path) => read_text_file(path),
        None => {
            print!("Enter text: ");
            read_user_input(reader)
        }
    }
}

pub fn read_text_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| String::new())
}


fn read_user_input<R: BufRead>(mut reader: R) -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    reader.read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{Cursor};
    use super::*;

    #[test]
    fn test_get_input_text_without_path() {
        let reader = Cursor::new(b"this is my input text");
        assert_eq!("this is my input text", get_input_text(&None,  reader))
    }

    #[test]
    fn test_get_input_text_with_non_existing_file() {
        assert_eq!("", get_input_text(&Some(&"file_does_not_exist.txt".to_string()), Cursor::new(b"This input should be ignored")))
    }

    #[test]
    fn test_get_input_text_with_file() {
        let mut file = File::create("test.txt").expect("cannot create file");
        file.write_all(b"this file has text.").expect("cannot write file");

        let actual = get_input_text(&Some(&"test.txt".to_string()), Cursor::new(b"this input should be ignored"));

        fs::remove_file("test.txt").expect("could not remove stopwords.txt");

        assert_eq!("this file has text.", actual)
    }
}