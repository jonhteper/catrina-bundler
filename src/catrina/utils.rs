use eyre::Result;
use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;

pub fn read_user_response() -> String {
    let mut user_response = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut user_response)
        .expect("Error reading user input");
    user_response.trim().to_string()
}

pub fn bin_dir() -> PathBuf {
    let bin = env::current_exe().expect("Error reading binary path");
    let dir = bin.parent().unwrap();
    PathBuf::from(dir)
}

pub fn getwd() -> PathBuf {
    env::current_dir().expect("Error reading execution path ")
}

pub fn file_to_string(mut file: File) -> Result<String> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
