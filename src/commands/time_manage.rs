use chrono::{DateTime, Duration, Local};
use colored::*;
use csv::{Reader, Writer};
use std::fs::OpenOptions;
use std::process;

pub fn time_manage(command: Vec<String>) {
    if command.len() != 2 {
        eprintln!("Usage: work <in|out>");
        process::exit(1);
    }

    let action = &command[1];

    match action.as_str() {
        "in" => work_start(),
        "out" => work_end(),
        _ => {
            eprintln!("Invalid action: {}", action);
            process::exit(1);
        }
    }
}

fn work_start() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("start_time_log.csv")
        .expect("Failed to open file");

    let mut wtr = Writer::from_writer(file);

    let now: DateTime<Local> = Local::now();
    wtr.write_record(&[now.to_rfc3339()])
        .expect("Failed to write to CSV");
    println!("Work started at {}", now.format("%Y/%m/%d %H:%M"));
}

fn work_end() {
    let work_log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("work_log.csv")
        .expect("Failed to open file");

    let start_log_file = OpenOptions::new()
        .read(true)
        .open("start_time_log.csv")
        .expect("Failed to open file");

    let mut wtr = Writer::from_writer(work_log_file);

    let now: DateTime<Local> = Local::now();
    let mut rdr = Reader::from_reader(start_log_file);

    let record = rdr
        .records()
        .last()
        .unwrap()
        .expect("Failed to read record");
    let last_record: String = record.iter().next().unwrap().to_string();

    let start_time =
        DateTime::parse_from_rfc3339(&last_record).expect("Failed to parse start time");
    let worked_duration: Duration = now.signed_duration_since(start_time);

    wtr.write_record(&[
        last_record.clone(),
        now.to_rfc3339(),
        format!(
            "{}:{:02}",
            worked_duration.num_hours(),
            worked_duration.num_minutes() % 60
        ),
    ])
    .expect("Failed to write to CSV");

    println!(
        "
                        ╭╯         ╭╯
                        ╰╮ Good   ╭╯    {} {}
                        ╭╯ Job! ╭╯
        ▓▓██████████▒ ╭━╯               {}🚬
        ",
        "worked for".bright_cyan().bold(),
        format!(
            "{}:{:02}",
            worked_duration.num_hours(),
            worked_duration.num_minutes() % 60
        )
        .bright_purple()
        .bold(),
        "Wanna go for a smoke?".bright_cyan().bold()
    );
}
