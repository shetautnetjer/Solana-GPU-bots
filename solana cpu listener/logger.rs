// logger.rs
use crossbeam::channel::Receiver;
use std::fs::OpenOptions;
use std::io::Write;

pub fn run_logger(rx: Receiver<String>) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("pool_updates.csv")
        .expect("Cannot open log file");

    while let Ok(line) = rx.recv() {
        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Logger error: {}", e);
        }
    }
}
