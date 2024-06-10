use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::{Child, Command, Stdio};
use serial_test::{parallel, serial};

#[test]
#[parallel]
fn test_main_little_lamb_poem() {
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("no output");

    {
        let stdin = child.stdin.as_mut().expect("no stdin received");
        stdin.write_all(b"marry had a little lamb").expect("stdin not writable");
    }

    assert_eq!("Enter text: Number of words: 5, unique: 5\n", read_output(child));
}

#[test]
#[parallel]
fn test_main_duplicate_words() {
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("no output");

    {
        let stdin = child.stdin.as_mut().expect("no stdin received");
        stdin.write_all(b"there are duplicates in this text, try to find them in the text!").expect("stdin not writable");
    }

    assert_eq!("Enter text: Number of words: 13, unique: 11\n", read_output(child));
}

#[test]
#[parallel]
fn test_main_words_with_hyphens() {
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("no output");

    {
        let stdin = child.stdin.as_mut().expect("no stdin received");
        stdin.write_all(b"Humpty-Dumpty sat on a wall. Humpty-Dumpty had a great fall.").expect("stdin not writable");
    }

    assert_eq!("Enter text: Number of words: 10, unique: 8\n", read_output(child));
}

#[test]
#[serial]
fn test_main_little_lamb_poem_with_stopwords() {
    let stop_words = br#"the
a
on
off
"#;

    let mut file = File::create("stopwords.txt").expect("cannot create file");
    file.write_all(stop_words).expect("cannot write file");

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("no output");

    {
        let stdin = child.stdin.as_mut().expect("no stdin received");
        stdin.write_all(b"marry had a little lamb").expect("stdin not writable");
    }

    let output  = read_output(child);
    fs::remove_file("stopwords.txt").expect("could not remove stopwords.txt");

    assert_eq!("Enter text: Number of words: 4, unique: 4\n", output)
}

#[test]
#[serial]
fn test_main_little_lamb_poem_in_file() {
    let poem = b"marry had a little lamb";

    let mut file = File::create("text.txt").expect("cannot create file");
    file.write_all(poem).expect("cannot write file");

    let child = Command::new("cargo")
        .arg("run")
        .arg("text.txt")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("no output");

    let output = read_output(child);
    fs::remove_file("text.txt").expect("could not remove stopwords.txt");

    assert_eq!("Number of words: 5, unique: 5\n", output);

}


fn read_output(child: Child) -> String {
    let enter_text_prompt = child.wait_with_output().expect("no stdout received");
    String::from(String::from_utf8_lossy(&enter_text_prompt.stdout))
}