use std::{env, fs};
use std::error::Error;
use std::path::PathBuf;
use std::str::Split;

const USE_TEST_INPUT: bool = false;

fn get_current_working_dir() -> PathBuf {
    return env::current_dir().unwrap();
}

pub fn read_file_in_cwd(path: String) -> String {
    match fs::read_to_string(get_current_working_dir().join(path.clone())) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file at path {} with {:?}", path, error),
    }
}

pub fn read_file_for_day(day: &str) -> String {
    return read_file_in_cwd(if USE_TEST_INPUT { format!("../input/day{}-test.txt", day) } else { format!("../input/day{}.txt", day) });
}

pub fn get_block_endings(input: &str) -> &str {
    return if input.contains("\r") { "\r\n\r" } else { "\n\n" };
}
pub fn get_line_endings(input: &str) -> &str {
    return if input.contains("\r") { "\r\n" } else { "\n" };
}
