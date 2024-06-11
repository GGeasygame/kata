use std::{env};
use std::collections::HashSet;
use std::io::{stdin, stdout};
use wordcount::text_stats::text_stats;
use wordcount::io::args::Args;
use wordcount::io::out;
use wordcount::io::input;

const STOPWORDS_FILE_PATH: &str = "./stopwords.txt";

fn main() {
    let arg_vec: Vec<_> = env::args().collect();
    let args: Args = Args::new(arg_vec);
    let input_text = input::get_input_text(&args.get_text_file_path(), stdin().lock());

    let stopwords: HashSet<_> = read_stopwords_file();

    let mut indexed_words: Option<Vec<String>> = None;
    if args.has_index_flag() {
        indexed_words = Option::from(text_stats::get_indexed_words(&input_text, &stopwords));
    }

    let dictionary: Option<HashSet<String>> = get_dictionary_from_arg(args.get_dictionary_file_path());

    out::print_result(&mut stdout(), text_stats::count_words(&input_text, &stopwords),
                 text_stats::count_unique_words(&input_text, &stopwords),
                 text_stats::calculate_average_characters_of_words(&input_text, &stopwords),
                 &indexed_words,
                 &dictionary)
}

fn read_stopwords_file() -> HashSet<String> {
    input::read_text_file(STOPWORDS_FILE_PATH).split("\n").into_iter().map(|s| s.to_string()).collect()
}

fn get_dictionary_from_arg(arg: Option<String>) -> Option<HashSet<String>> {
    arg.map_or(None, |file_path| {
        Some(read_dictionary_file(&file_path))
    })
}

fn read_dictionary_file(dictionary_file_path: &String) -> HashSet<String> {
    input::read_text_file(dictionary_file_path).split("\n").into_iter().map(|s| s.to_string()).collect()
}
