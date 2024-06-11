use std::fs;
use regex::Regex;

pub struct Args {
    args: Vec<String>,
}

impl Args {
    pub fn new(args: Vec<String>) -> Self {
        Args { args }
    }

    pub fn get_text_file_path(&self) -> Option<&String> {
        for arg in &self.args[1..] {
            if fs::metadata(arg).is_ok() {
                return Some(arg);
            }
        }
        None
    }

    pub fn has_index_flag(&self) -> bool {
        for arg in &self.args[1..] {
            if arg.eq("-index") {
                return true;
            }
        }
        false
    }

    pub fn get_dictionary_file_path(&self) -> Option<String> {
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