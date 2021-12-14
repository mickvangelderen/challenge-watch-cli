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
    let test_dir_path = fs::canonicalize(test_dir.path()).expect("Failed to canonicalize temp dir"); // necessary on MacOS to prepend `/private` to the `/var` path created by TempDir?.
    let mut child = Command::new(exe_path())
        .arg(&test_dir_path)
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
    let deep_dir_path = test_dir_path.join("deep");
    fs::create_dir(&deep_dir_path).expect("Failed to create deep dir!");
    let line = rx
        .recv_timeout(time::Duration::from_secs(1))
        .expect("Did not receive line in time!");
    assert_eq!(line, format!("CREATE {:?}", deep_dir_path.display()));

    // Create a file and wait on a create line being printed.
    let file_txt_path = deep_dir_path.join("file.txt");
    fs::write(&file_txt_path, "Programming is cool!").expect("Failed to create file.txt!");

    if cfg!(target_os = "macos") {
        let line = rx
            .recv_timeout(time::Duration::from_secs(1))
            .expect("Did not receive line in time!");
        assert_eq!(
            line,
            format!("CREATE | WRITE {:?}", file_txt_path.display())
        );
    } else {
        let line = rx
            .recv_timeout(time::Duration::from_secs(1))
            .expect("Did not receive line in time!");
        assert_eq!(line, format!("CREATE {:?}", file_txt_path.display()));
        let line = rx
            .recv_timeout(time::Duration::from_secs(1))
            .expect("Did not receive line in time!");
        assert_eq!(line, format!("WRITE {:?}", file_txt_path.display()));
    }

    let line = rx
        .recv_timeout(time::Duration::from_secs(1))
        .expect("Did not receive line in time!");
    assert_eq!(line, format!("WRITE {:?}", deep_dir_path.display()));

    child.kill().expect("Failed to kill child!"); // Might want to rename this variable.

    child.wait().expect("Failed to wait for child to exit!");
}
