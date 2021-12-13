use std::process::Command;

const CLI: &str = "challenge_watch_cli";

#[test]
fn display_help() {
    let output = Command::new(CLI)
        .arg("--help")
        .output()
        .expect("Failed to run command!");
    assert!(output.status.success());
}

#[test]
fn no_path() {
    let output = Command::new(CLI).output().expect("Failed to run command!");
    assert_eq!(String::from_utf8(output.stderr).expect("stderr is not valid utf-8!"), "error: The following required arguments were not provided:\n    <PATH>...\n\nUSAGE:\n    challenge_watch_cli <PATH>...\n\nFor more information try --help\n");
    assert!(!output.status.success());
}
