use std::process::Command;

#[test]
fn it_works() {
    let output = Command::new("cargo")
        .args(["run", "body", "poem.txt"])
        .output()
        .expect("Failed to run minigrep!");

    let r = String::from_utf8(output.stdout).unwrap();

    assert_eq!(r,"Found 3 Matches for `body` in poem.txt:\n1. I'm nobody! Who are you?\n2. Are you nobody, too?\n6. How dreary to be somebody!\n")
}
