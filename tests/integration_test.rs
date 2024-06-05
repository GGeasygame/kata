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

    assert_output(child);
}

fn assert_output(child: Child) {
    let enter_text_prompt = child.wait_with_output().expect("no stdout received");
    let out = String::from_utf8_lossy(&enter_text_prompt.stdout);

    assert_eq!("Enter text: Number of words: 5\n", out);
}