use std::io::{self, BufRead, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Barrier};
use std::time::Duration;
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
    
    // NOTE(mickvangelderen): This canonicalize is necessary to prepend `/private` to the temp path returned by `TempDir.path()` on MacOS.
    let test_dir_path = fs::canonicalize(test_dir.path()).expect("Failed to canonicalize temp dir"); 

    let mut child = Command::new(exe_path())
        .arg(&test_dir_path)
        .env("RUST_LOG", "")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run command!");

    let (tx, rx) = mpsc::channel();

    let _ = std::thread::spawn({
        let stdout = child.stdout.take().unwrap();
        let reader = io::BufReader::new(stdout);
        let lines = reader.lines();

        move || {
            for line in lines {
                let line = line.expect("Failed to read line form stdout!");
                if let Err(_) = tx.send(line) {
                    // Stop reading if we can not send anymore.
                    break;
                }
            }
        }
    });

    // NOTE(mickvangelderen): This thread::sleep is the difference between the `recv_timeout` timing out and properly receiving the first line from the spawned process' stdout.
    // My best guess is that the file is created before the spawned process has started watching the folder.
    // That could be fixed by outputting a line after the watcher is initialized and waiting on it here.
    // For this assignment I will concede and leave things here.
    thread::sleep(Duration::from_secs(1));

    let read_line = || {
        rx.recv_timeout(time::Duration::from_secs(1))
            .expect("Did not receive line in time!")
    };

    // Create a directory.
    let deep_dir_path = test_dir_path.join("deep");
    fs::create_dir(&deep_dir_path).expect("Failed to create deep dir!");

    // Wait on a create line being printed.
    assert_eq!(read_line(), format!("CREATE {:?}", deep_dir_path.display()));

    // Create a file and wait on a create line being printed.
    let file_txt_path = deep_dir_path.join("file.txt");
    fs::write(&file_txt_path, "Programming is cool!").expect("Failed to create file.txt!");

    // Wait on create and write events being printed.
    if cfg!(target_os = "macos") {
        assert_eq!(
            read_line(),
            format!("CREATE | WRITE {:?}", file_txt_path.display())
        );
    } else {
        assert_eq!(read_line(), format!("CREATE {:?}", file_txt_path.display()));
        assert_eq!(read_line(), format!("WRITE {:?}", file_txt_path.display()));
    }

    if cfg!(target_os = "linux") {
        assert_eq!(read_line(), format!("CLOSE_WRITE {:?}", file_txt_path.display()));
    }

    assert_eq!(read_line(), format!("WRITE {:?}", deep_dir_path.display()));

    child.kill().expect("Failed to kill child!"); // Might want to rename this variable.

    child.wait().expect("Failed to wait for child to exit!");
}
