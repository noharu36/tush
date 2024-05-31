use csv::Writer;
use std::fs::OpenOptions;
use std::process;

pub fn exit() {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("start_time_log.csv")
        .expect("Failed to open file");

    let mut wtr = Writer::from_writer(file);
    wtr.write_record(["start"]).expect("Failed to write to CSV");
    wtr.flush().ok().unwrap();

    process::exit(0)
}
