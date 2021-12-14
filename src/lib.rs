use log::info;
use notify::{raw_watcher, Error, RawEvent, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;

const SHUTDOWN_EVENT_NAME: &str = "SHUTDOWN";
/// The returned event is used to signal to whoever is listening to the events from the file watcher that we want to shut the application down.
fn shutdown_event() -> RawEvent {
    RawEvent {
        path: None,
        op: Err(Error::Generic(SHUTDOWN_EVENT_NAME.to_owned())),
        cookie: None,
    }
}

fn is_shutdown_event(event: &RawEvent) -> bool {
    if let &RawEvent {
        path: None,
        op: Err(Error::Generic(ref err)),
        cookie: None,
    } = event
    {
        err == SHUTDOWN_EVENT_NAME
    } else {
        false
    }
}

fn set_ctrlc_handler(tx: mpsc::Sender<RawEvent>) {
    ctrlc::set_handler(move || {
        info!("Initiating shutdown.");
        tx.send(shutdown_event())
            .expect("Failed to send shutdown signal over watcher channnel!");
    })
    .expect("Failed to set SIGINT handler!");
}

fn print_event(op: notify::Op, path: std::path::PathBuf, cookie: Option<u32>) {
    print!("{:?} {:?}", op, path);
    if let Some(cookie) = cookie {
        print!(" {:?}", cookie);
    }
    println!();
}

pub fn watch<'a, I: IntoIterator<Item = &'a Path>>(paths: I) {
    let (tx, rx) = mpsc::channel();

    set_ctrlc_handler(tx.clone());

    let mut watcher = raw_watcher(tx).unwrap();

    for path in paths {
        info!("Watching {:?}.", path.display());
        watcher
            .watch(path, RecursiveMode::Recursive)
            .expect("Failed to watch one of the paths!");
    }

    loop {
        match rx.recv() {
            Ok(ref event) if is_shutdown_event(event) => {
                break;
            }
            Ok(RawEvent {
                path: Some(path),
                op: Ok(op),
                cookie,
            }) => {
                print_event(op, path, cookie);
            }
            Ok(event) => {
                eprintln!("Receive incomplete event: {:?}!", event);
            }
            Err(error) => {
                eprintln!("Received watch error: {:?}!", error);
            }
        }
    }
}
