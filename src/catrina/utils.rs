use eyre::Result;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::path::PathBuf;

/// Return user input in standard input.
pub fn read_user_response() -> String {
    let mut user_response = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut user_response)
        .expect("Error reading user input");
    user_response.trim().to_string()
}

/// Return the binary directory.
pub fn bin_dir() -> PathBuf {
    let bin = env::current_exe().expect("Error reading binary path");
    let dir = bin.parent().unwrap();
    PathBuf::from(dir)
}

/// Return the current directory
pub fn getwd() -> PathBuf {
    env::current_dir().expect("Error reading execution path ")
}

/// Return the file content in a String
pub fn file_to_string(mut file: File) -> Result<String> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Return a random name
pub fn random_name(len: usize) -> String {
    let rand_name: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();

    rand_name
}
