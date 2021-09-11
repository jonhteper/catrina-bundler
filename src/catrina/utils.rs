use eyre::Result;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};
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

/// Read a file and return lines in a Vec<String>
/// Based in https://dev.to/dandyvica/different-ways-of-reading-files-in-rust-2n30
pub fn file_to_vec_string(path: &PathBuf) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(&file);
    let mut content: Vec<String> = vec![];
    let mut line = String::new();

    loop {
        let bytes = reader.read_line(&mut line)?;

        // EOF: save last file address to restart from this address for next run
        if bytes == 0 {
            break;
        }

        content.push(line.clone());

        // do not accumulate data
        line.clear();
    } //loop

    Ok(content)
}

/// append vec string in a file, use with file_to_vec_string
pub fn write_vec_string_in_file(file_path: &PathBuf, content: Vec<String>) -> Result<()> {
    if content.len() > 0 {
        let mut file = OpenOptions::new().append(true).open(&file_path)?;
        for line in content {
            file.write_all(line.as_bytes())?;
        }
    }

    Ok(())
}
