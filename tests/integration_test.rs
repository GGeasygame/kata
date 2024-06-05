use std::fs::File;
use std::io::Write;
use std::process::{Child, Command, Stdio};

#[test]
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

    assert_output("Enter text: Number of words: 5\n", child);
}

#[test]
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

    assert_output("Enter text: Number of words: 4\n", child);
}

fn assert_output(expected: &str, child: Child) {
    let enter_text_prompt = child.wait_with_output().expect("no stdout received");
    let out = String::from_utf8_lossy(&enter_text_prompt.stdout);

    assert_eq!(expected, out);
}