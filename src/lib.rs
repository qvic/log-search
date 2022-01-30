use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs::File;
use std::os::unix::fs::FileExt;

use chrono::NaiveDateTime;

pub trait RandomAccess {
    fn read_at_position(&self, i: u64) -> Option<char>;
}

impl RandomAccess for String {
    fn read_at_position(&self, i: u64) -> Option<char> {
        self.chars().collect::<Vec<char>>().get(i as usize).copied()
    }
}

impl RandomAccess for File {
    fn read_at_position(&self, i: u64) -> Option<char> {
        let mut buf = [0u8; 4];
        let n = self.read_at(&mut buf, i).unwrap();
        if n > 0 {
            std::str::from_utf8(&buf).unwrap().chars().next()
        } else {
            None
        }
    }
}

pub fn binary_search_line<T, F>(source: &T, char_count: u64, check_line: F) -> Result<Option<String>, String>
    where T: RandomAccess, F: Fn(&str) -> Result<Ordering, String> {
    let mut size = char_count;
    if size == 0 {
        return Ok(None);
    }
    let mut base = 0;

    while size > 1 {
        let half = size / 2;
        let mid = base + half;
        let line = find_line_by_position(source, mid).expect(format!("Line not found for position {}", mid).as_str());
        let ordering = check_line(&line)?;
        match ordering {
            Ordering::Less => base = mid,
            Ordering::Equal => return Ok(Some(line)),
            Ordering::Greater => {}
        }
        size -= half;
    }

    Ok(None)
}

fn find_line_by_position<T: RandomAccess>(source: &T, position: u64) -> Option<String> {
    let mut buffer: VecDeque<char> = VecDeque::new();
    let mut i = position;
    loop {
        match source.read_at_position(i) {
            Some('\n') => break,
            Some(c) => buffer.push_back(c),
            None => break
        }
        i += 1;
    }

    if position > 0 {
        i = position - 1;
        loop {
            match source.read_at_position(i) {
                Some('\n') => break,
                Some(c) => buffer.push_front(c),
                None => break
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }

    if buffer.is_empty() {
        None
    } else {
        Some(buffer.into_iter().collect())
    }
}

fn parse_date(date: &str, date_format: &str) -> Result<NaiveDateTime, String> {
    NaiveDateTime::parse_from_str(date, date_format).map_err(|e| e.to_string())
}

pub fn compare_by_datetime(line: &str, delimiter: &str,
                           target_date_str: &str, date_format: &str) -> Result<Ordering, String> {
    let date = line.split_once(delimiter).ok_or(format!("Found badly formatted line: {}", line))?.0;
    let parsed_date = parse_date(date, date_format)?;
    let target_date = parse_date(target_date_str, date_format)?;
    Ok(parsed_date.cmp(&target_date))
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_find_line_with_position() {
        let string = String::from("1 - Hello\n2 - World\n3 - And goodbye\n\n");
        assert_eq!(find_line_by_position(&string, 0), Some(String::from("1 - Hello")));
        assert_eq!(find_line_by_position(&string, 5), Some(String::from("1 - Hello")));
        assert_eq!(find_line_by_position(&string, 9), Some(String::from("1 - Hello")));

        assert_eq!(find_line_by_position(&string, 10), Some(String::from("2 - World")));
        assert_eq!(find_line_by_position(&string, 18), Some(String::from("2 - World")));
        assert_eq!(find_line_by_position(&string, 19), Some(String::from("2 - World")));

        assert_eq!(find_line_by_position(&string, 20), Some(String::from("3 - And goodbye")));
        assert_eq!(find_line_by_position(&string, 30), Some(String::from("3 - And goodbye")));
        assert_eq!(find_line_by_position(&string, 35), Some(String::from("3 - And goodbye")));

        assert_eq!(find_line_by_position(&string, 36), None);
    }

    #[test]
    fn test_binary_search_line() {
        let string = String::from("1 - Lorem\n2 - Ipsum\n3 - Dolor sit amet\n4 - Consectetur adipiscing elit\n5 - Excepteur sint");
        let length = string.len() as u64;

        let result = binary_search_line(&string, length,
                                        |x| compare_by_bullet_number(x, 4));
        assert_eq!(result, Ok(Some(String::from("4 - Consectetur adipiscing elit"))));
    }

    fn compare_by_bullet_number(line: &str, n: usize) -> Result<Ordering, String> {
        let delimiter = " - ";
        let prefix = line.split_once(delimiter).ok_or(format!("Found badly formatted line: {}", line))?.0;
        let parsed_number = prefix.parse::<usize>().map_err(|e| e.to_string())?;
        Ok(parsed_number.cmp(&n))
    }

    #[test]
    fn test_binary_search_line_date() {
        let string = String::from("2020-01-01 Lorem\n2020-01-05 Ipsum\n2020-01-10 Dolor sit amet\n2020-01-11 Excepteur sint");
        let length = string.len() as u64;
        let result = binary_search_line(&string, length,
                                        |x| compare_by_date(x, NaiveDate::from_ymd(2020, 1, 5)));
        assert_eq!(result, Ok(Some(String::from("2020-01-05 Ipsum"))));
    }

    fn compare_by_date(line: &str, date: NaiveDate) -> Result<Ordering, String> {
        let delimiter = " ";
        let date_format = "%Y-%m-%d";
        let date_prefix = line.split_once(delimiter).ok_or(format!("Found badly formatted line: {}", line))?.0;
        let parsed_date = NaiveDate::parse_from_str(date_prefix, date_format).map_err(|e| e.to_string())?;
        Ok(parsed_date.cmp(&date))
    }

    #[test]
    fn test_binary_search_line_datetime() {
        let date_format = "%Y-%m-%d %H:%M:%S";
        let delimiter = " - ";
        let string = String::from("2020-01-01 14:27:28 - Lorem\n2020-01-01 18:59:15 - Ipsum\n2020-01-02 01:17:24 - Dolor sit amet");
        let length = string.len() as u64;
        let result = binary_search_line(&string, length,
                                        |line| compare_by_datetime(line, delimiter, "2020-01-01 14:27:28", date_format));
        assert_eq!(result, Ok(Some(String::from("2020-01-01 14:27:28 - Lorem"))));
    }
}