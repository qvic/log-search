use std::env;
use std::fs::File;
use std::time::Instant;

use crate::lib::{binary_search_line, compare_by_datetime};

mod lib;

fn main() -> std::io::Result<()> {
    const SEARCH_TARGET: &'static str = "2000-01-02 15:46:40";
    const FILE_PATH: &'static str = "log.txt";
    let date_format = "%Y-%m-%d %H:%M:%S";
    let delimiter = " - ";

    let file = File::open(FILE_PATH)?;
    let file_size = file.metadata().unwrap().len();
    println!("File path: {}", FILE_PATH);
    println!("File size: {}", file_size);

    let start_time = Instant::now();
    let result = binary_search_line(&file, file_size,
                                    |line| compare_by_datetime(line, delimiter, SEARCH_TARGET, date_format))
        .unwrap();
    let elapsed_time = start_time.elapsed().as_millis();

    println!("Execution took {} ms", elapsed_time);
    match result {
        Some(line) => println!("Found match '{}'", line),
        None => println!("Match not found for pattern '{}'", SEARCH_TARGET)
    };

    Ok(())
}
