use std::fs;
use std::fs::File;
use std::io::{Write};
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

    assert_eq!("Enter text: Number of words: 5, unique: 5; average word length: 3.80 characters\nEnter text: ", read_output(child));
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

    assert_eq!("Enter text: Number of words: 13, unique: 11; average word length: 3.85 characters\nEnter text: ", read_output(child));
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

    assert_eq!("Enter text: Number of words: 10, unique: 8; average word length: 4.90 characters\nEnter text: ", read_output(child));
}

#[test]
#[parallel]
fn test_main_with_index_arg() {
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("main.rs")
        .arg("-index")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("no output");

    {
        let stdin = child.stdin.as_mut().expect("no stdin received");
        stdin.write_all(b"marry had a little lamb").expect("stdin not writable");
    }

    assert_eq!(r#"Enter text: Number of words: 5, unique: 5; average word length: 3.80 characters
Index:
a
had
lamb
little
marry
Enter text: "#,
               read_output(child));
}

#[test]
#[parallel]
fn test_main_multiple_inputs() {
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
        stdin.flush()
    }.expect("could not write to stdin");

    {
        let stdin = child.stdin.as_mut().expect("no stdin received");
        stdin.write_all(b"this is the second text").expect("stdin not writable");
        stdin.flush()
    }.expect("could not write to stdin");

    assert_eq!("Enter text: Number of words: 5, unique: 5; average word length: 3.80 characters\nEnter text: ", read_output(child));
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

    let output = read_output(child);
    fs::remove_file("stopwords.txt").expect("could not remove stopwords.txt");

    assert_eq!("Enter text: Number of words: 4, unique: 4; average word length: 4.50 characters\nEnter text: ", output)
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

    assert_eq!("Number of words: 5, unique: 5; average word length: 3.80 characters", output);
}

#[test]
#[serial]
fn test_main_with_index_arg_and_file_and_stopwords() {
    let stop_words = br#"the
a
on
off
"#;

    let poem = b"marry had a little lamb";

    let mut file = File::create("text.txt").expect("cannot create file");
    file.write_all(poem).expect("cannot write file");


    let mut file = File::create("stopwords.txt").expect("cannot create file");
    file.write_all(stop_words).expect("cannot write file");

    let child = Command::new("cargo")
        .arg("run")
        .arg("main.rs")
        .arg("text.txt")
        .arg("-index")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("no output");

    let output = read_output(child);
    fs::remove_file("stopwords.txt").expect("could not remove stopwords.txt");
    fs::remove_file("text.txt").expect("could not remove stopwords.txt");

    assert_eq!(r#"Number of words: 4, unique: 4; average word length: 4.50 characters
Index:
had
lamb
little
marry"#, output)
}

#[test]
#[serial]
fn test_main_with_index_arg_and_dictionary() {
    let dict = br#"big
small
little
cat
dog
have
has
had"#;

    let mut file = File::create("dict.txt").expect("cannot create file");
    file.write_all(dict).expect("cannot write file");

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("main.rs")
        .arg("-index")
        .arg("-dictionary=dict.txt")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("no output");

    {
        let stdin = child.stdin.as_mut().expect("no stdin received");
        stdin.write_all(b"marry had a little lamb").expect("stdin not writable");
    }

    let output = read_output(child);

    fs::remove_file("dict.txt").expect("could not remove stopwords.txt");

    assert_eq!(r#"Enter text: Number of words: 5, unique: 5; average word length: 3.80 characters
Index (unknown: 3):
a*
had
lamb*
little
marry*
Enter text: "#,
               output);
}

#[test]
#[serial]
fn test_main_with_dictionary() {
    let dict = br#"
big
small
little
cat
dog
have
has
had"#;

    let mut file = File::create("dict.txt").expect("cannot create file");
    file.write_all(dict).expect("cannot write file");

    let mut child = Command::new("cargo")
        .arg("run")
        .arg("main.rs")
        .arg("-dictionary=dict.txt")
        .arg("--quiet")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("no output");

    {
        let stdin = child.stdin.as_mut().expect("no stdin received");
        stdin.write_all(b"marry had a little lamb").expect("stdin not writable");
    }

    let output = read_output(child);
    fs::remove_file("dict.txt").expect("could not remove stopwords.txt");

    assert_eq!("Enter text: Number of words: 5, unique: 5; average word length: 3.80 characters\nEnter text: ",
               output);
}

fn read_output(child: Child) -> String {
    let enter_text_prompt = child.wait_with_output().expect("no stdout received");
    String::from(String::from_utf8_lossy(&enter_text_prompt.stdout))
}