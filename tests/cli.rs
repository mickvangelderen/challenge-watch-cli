use std::path::Path;
use std::process::Command;

fn exe_path() -> &'static Path {
    std::path::Path::new(env!("CARGO_BIN_EXE_challenge_watch_cli"))
}

#[test]
fn display_help() {
    let output = Command::new(exe_path())
        .arg("--help")
        .output()
        .expect("Failed to run command!");
    assert!(output.status.success());
}

#[test]
fn no_path() {
    let output = Command::new(exe_path())
        .output()
        .expect("Failed to run command!");
    let stderr = String::from_utf8(output.stderr).expect("stderr is not valid utf-8!");
    assert!(stderr.starts_with(
        "error: The following required arguments were not provided:\n    <PATH>...\n"
    ));
    assert!(!output.status.success());
}
