use std::env;
use std::fs::File;
use std::time::Instant;

use crate::lib::{binary_search_line, compare_by_datetime};

mod lib;

fn main() -> std::io::Result<()> {
    let file_path = env::args().nth(1).expect("No log file path given");
    let date_format = env::args().nth(2).expect("No date format given (e.g. '%Y-%m-%d %H:%M:%S')");
    let date_delimiter = env::args().nth(3).expect("No date delimiter given");
    let target_date = env::args().nth(4).expect("No target date given");

    println!("File path: '{}'", file_path);
    println!("Date format: '{}'", date_format);
    println!("Date delimiter: '{}'", date_delimiter);
    println!("Target date: '{}'", target_date);

    let file = File::open(file_path)?;
    let file_size = file.metadata()?.len();

    println!("File size: {} bytes", file_size);

    let start_time = Instant::now();
    let result = binary_search_line(&file, file_size,
                                    |line| compare_by_datetime(line, &date_delimiter, &target_date, &date_format))
        .unwrap();
    let elapsed_time = start_time.elapsed().as_millis();

    println!();
    println!("Execution took {} ms", elapsed_time);
    match result {
        Some(line) => println!("Found match '{}'", line),
        None => println!("Match not found for pattern '{}'", target_date)
    };

    Ok(())
}
