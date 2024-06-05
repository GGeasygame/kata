use std::io::{stdin, stdout, Write};
use wordcount::count_words;

fn main() {
    let mut s = String::new();
    print!("Enter text: ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    println!("Number of words: {}", count_words(&s))
}
