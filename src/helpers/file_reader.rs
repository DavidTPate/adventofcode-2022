use std::{env, fs};
use std::path::PathBuf;

fn get_current_working_dir() -> PathBuf {
    return env::current_dir().unwrap();
}

pub fn read_file_in_cwd(file: &str) -> String {
    return fs::read_to_string(get_current_working_dir().join(file)).unwrap();
}