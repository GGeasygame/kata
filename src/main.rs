use std::{env, fs};
use std::collections::HashSet;
use std::io::{stdin, stdout, Write};
use regex::Regex;

use wordcount::{calculate_average_characters_of_words, count_unique_words, count_words, get_indexed_words};


const STOPWORDS_FILE_PATH: &str = "./stopwords.txt";

struct Args {
    args: Vec<String>,
}

impl Args {
    fn new(args: Vec<String>) -> Self {
        Args { args }
    }

    fn get_text_file_path(&self) -> Option<&String> {
        for arg in &self.args[1..] {
            if fs::metadata(arg).is_ok() {
                return Some(arg);
            }
        }
        None
    }

    fn has_index_flag(&self) -> bool {
        for arg in &self.args[1..] {
            if arg.eq("-index") {
                return true;
            }
        }
        false
    }

    fn get_dictionary_file_path(&self) -> Option<String> {
        let regex = Regex::new(r"-dictionary=.+").unwrap();
        for arg in &self.args[1..] {
            if regex.is_match(arg) {
                let path = arg.split("=").collect::<Vec<_>>()[1];
                if fs::metadata(path).is_ok() {
                    return Some(path.to_string());
                }
                return None
            }
        }
        None
    }
}

fn main() {
    let arg_vec: Vec<_> = env::args().collect();
    let args: Args = Args::new(arg_vec);
    let input_text = get_input_text(&args.get_text_file_path());

    let stopwords: HashSet<_> = read_stopwords_file();

    let mut indexed_words: Option<Vec<String>> = None;
    if args.has_index_flag() {
        indexed_words = Option::from(get_indexed_words(&input_text, &stopwords, &get_dictionary_from_arg(args.get_dictionary_file_path())));
    }

    print!("{}", get_output(count_words(&input_text, &stopwords),
                            count_unique_words(&input_text, &stopwords),
                            calculate_average_characters_of_words(&input_text, &stopwords),
                            &indexed_words));
}

fn get_output(amount_words: i32, amount_unique_words: i32, average_characters_of_words: f32, indexed_words: &Option<Vec<String>>) -> String {
    let mut output = format!("Number of words: {}, unique: {}; average word length: {:.2} characters",
                             amount_words, amount_unique_words, average_characters_of_words);
    if let Some(words) = indexed_words {
        output += "\nIndex:";
        let indexes: String = words.iter().map(|word| format!("\n{}", word)).collect();
        output += &indexes
    }
    output
}


fn get_input_text(text_file_path: &Option<&String>) -> String {
    match text_file_path {
        Some(path) => read_text_file(path),
        None => {
            print!("Enter text: ");
            read_user_input()
        }
    }
}

fn read_text_file(path: &str) -> String {
    dbg!(path);
    fs::read_to_string(path).unwrap_or_else(|_| String::new())
}

fn read_stopwords_file() -> HashSet<String> {
    read_text_file(STOPWORDS_FILE_PATH).split("\n").into_iter().map(|s| s.to_string()).collect()
}

fn get_dictionary_from_arg(arg: Option<String>) -> Option<HashSet<String>> {
    arg.map_or(None, |file_path| {
        Some(read_dictionary_file(&file_path))
    })
}

fn read_dictionary_file(dictionary_file_path: &String) -> HashSet<String> {
    read_text_file(dictionary_file_path).split("\n").into_iter().map(|s| s.to_string()).collect()
}

fn read_user_input() -> String {
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}
