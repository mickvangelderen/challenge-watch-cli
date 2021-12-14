use std::io::{self, BufRead, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::{fs, thread, time};

use tempdir::TempDir;

fn exe_path() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_challenge_watch_cli"))
}

#[test]
fn display_help() {
    let output = Command::new(exe_path())
        .arg("--help")
        .env("RUST_LOG", "")
        .output()
        .expect("Failed to run command!");
    assert!(output.status.success());
}

#[test]
fn no_path() {
    let output = Command::new(exe_path())
        .env("RUST_LOG", "")
        .output()
        .expect("Failed to run command!");
    let stderr = String::from_utf8(output.stderr).expect("stderr is not valid utf-8!");
    assert!(stderr.starts_with(
        "error: The following required arguments were not provided:\n    <PATH>...\n"
    ));
    assert!(!output.status.success());
}

#[test]
fn one_path() {
    let test_dir = TempDir::new("one_path").unwrap();

    let mut child = Command::new(exe_path())
        .arg(test_dir.path())
        .env("RUST_LOG", "")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run command!");

    let (tx, rx) = mpsc::channel();

    let _ = std::thread::spawn({
        let stdout = child.stdout.take().unwrap();
        let reader = io::BufReader::new(stdout);
        let lines = reader.lines();
        move || {
            println!("About to read lines!");
            for line in lines {
                println!("{:?}", line);
                let line = line.expect("Failed to read line form stdout!");
                println!("Received {:?}", line);
                if let Err(_) = tx.send(line) {
                    // Stop reading if we can not send anymore.
                    break;
                }
            }
        }
    });

    thread::sleep(time::Duration::from_secs(1));

    // Create a directory and wait on a create line being printed.
    let deep_dir = test_dir.path().join("deep");
    fs::create_dir(&deep_dir).expect("Failed to create deep dir!");
    let create_dir_line = rx
        .recv_timeout(time::Duration::from_secs(1))
        .expect("Did not receive line in time!");
    assert_eq!(create_dir_line, format!("CREATE {:?}", deep_dir.display()));

    // Create a file and wait on a create line being printed.
    let file_txt_path = deep_dir.join("file.txt");
    fs::write(&file_txt_path, "Programming is cool!").expect("Failed to create file.txt!");
    let create_file_line = rx
        .recv_timeout(time::Duration::from_secs(1))
        .expect("Did not receive line in time!");
    assert_eq!(
        create_file_line,
        format!("CREATE {:?}", file_txt_path.display())
    );

    child.kill().expect("Failed to kill child!"); // Might want to rename this variable.

    child.wait().expect("Failed to wait for child to exit!");
}
