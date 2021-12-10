use std::path::Path;

use challenge_watch_cli::watch;
use clap::{App, Arg};

fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", concat!(env!("CARGO_PKG_NAME"), "=WARN"));
    }
    pretty_env_logger::init();

    const ARG_PATH: &str = "path";

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("Mick van Gelderen <mickvangelderen@gmail.com>")
        .about("CLI app built for an interview.")
        .arg(
            Arg::with_name(ARG_PATH)
                .value_name("PATH")
                .help("Specifies the directory path to watch.")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let paths = matches
        .values_of_os(ARG_PATH)
        .expect("Expected at least one path!");

    watch(paths.map(Path::new))
}
