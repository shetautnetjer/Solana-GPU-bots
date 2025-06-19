// -------- src/logger.rs ---------------------------------------
use crate::types::types::LogRow;
use crossbeam::channel::Receiver;
use csv::Writer;
use std::fs::OpenOptions;

pub fn run_logger(rx: Receiver<LogRow>, path: &str) {
    // Create or append CSV file
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("Unable to open CSV");

    let mut wtr = Writer::from_writer(file);

    // Write header if file is new (csv::Writer handles duplicates benignly)
    let _ = wtr.write_record([
        "timestamp",
        "slot",
        "account",
        "mint",
        "owner",
        "ui_amount",
        "delta",
        "rolling_delta",
    ]);

    for row in rx {
        if let Err(e) = wtr.serialize(&row) {
            eprintln!("CSV write error: {e}");
        }
    }
}
