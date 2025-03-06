use chrono::{DateTime, TimeDelta, Local};
use colored::*;
use csv::{Reader, Writer};
use std::fs::OpenOptions;
use std::process;
use std::error::Error;

pub fn time_manage(command: Vec<String>) {
    if command.len() != 2 {
        eprintln!("Usage: work <in|out>");
        process::exit(1);
    }

    let action = &command[1];

    match action.as_str() {
        "in" => work_start(),
        "out" => work_end(),
        "ed" => calc_working_time().ok().unwrap(),
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
    let worked_duration: TimeDelta = now.signed_duration_since(start_time);

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

fn calc_working_time() -> Result<(), Box<dyn Error>> {
    let work_log_file = OpenOptions::new()
        .read(true)
        .open("work_log.csv")
        .expect("Failed to open file");

    let mut rdr = Reader::from_reader(work_log_file);

    let mut total_worked_time = TimeDelta::zero();
    for result in rdr.records() {
        let record = result?;
        let worked_time_str = record.get(2).unwrap();

        let parts: Vec<i64> = worked_time_str
            .split(':').collect::<Vec<&str>>()
            .iter()
            .map(|n| n.parse().unwrap())
            .collect();

        total_worked_time += TimeDelta::try_hours(parts[0]).unwrap() + TimeDelta::try_minutes(parts[1]).unwrap();

    }

    let total_hours = total_worked_time.num_hours();
    let total_minutes = total_worked_time.num_minutes() - total_hours * 60;

    let total_salary = total_hours * 1500 + total_minutes * 25;

    println!("Total worked time: {}:{}, Total Salary: {}", total_hours, total_minutes, total_salary);
    Ok(())
}
