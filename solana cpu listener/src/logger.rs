// -------- src/logger.rs ---------------------------------------
use crate::types::types::LogRow;
use crossbeam::channel::Receiver;
use csv::Writer;
use std::fs::OpenOptions;

pub fn run_logger(rx: Receiver<LogRow>) {
    // Create or append CSV file
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("pool_updates.csv")
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
    ]);

    for row in rx {
        if let Err(e) = wtr.serialize(&row) {
            eprintln!("CSV write error: {e}");
        }
    }
}
