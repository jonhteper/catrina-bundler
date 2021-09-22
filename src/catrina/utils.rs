use eyre::{Result, WrapErr};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{stdin, stdout, BufRead, BufReader, Read, Write};
use std::path::PathBuf;

pub const FILE_TO_VEC_ERR_MSJ: &str = "Error in file-to-vec conversion";

/// Return user input in standard input.
pub fn read_user_response() -> String {
    let mut user_response = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut user_response)
        .expect("Error reading user input");
    user_response.trim().to_string()
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

///
///
/// # Arguments
///
/// * `file_path`: file PathBuf where will write content
/// * `content`: vector whit content
/// * `condition`: function to decide which strings will write in final file.
///                 If returns true, line will be write
///
/// returns: Result<(), Report> from [eyre](https://github.com/yaahc/eyre) crate
///
/// # Examples
///
/// ```
///     // The next function copy all content
///     pub fn write_all_lines(file_path: &PathBuf, content: Vec<String>) -> Result<()> {
///         conditional_write_vec_string_in_file(&file_path, &content, |a,b| {
///             true
///         })
///     }
///
///     // The next function copy only pair lines
///     pub fn write_pair_lines(file_path: &PathBuf, content: Vec<String>) -> Result<()> {
///         conditional_write_vec_string_in_file(&file_path, &content, |a,b| {
///             a%2 == 0
///         })
///     }
/// ```
pub fn conditional_write_vec_string_in_file(
    file_path: &PathBuf,
    content: &Vec<String>,
    condition: fn(usize, &String) -> bool,
) -> Result<()> {
    if content.len() > 0 {
        let mut file = OpenOptions::new().append(true).open(&file_path)?;
        let content = content.clone();
        for (i, line) in content.iter().enumerate() {
            if condition(i, line) {
                file.write_all(line.as_bytes())?;
            }
        }
        file.write_all("\n".as_bytes())?; // force new line for new appends
    }

    Ok(())
}

/// append vec string in a file, use with file_to_vec_string
pub fn write_vec_string_in_file(file_path: &PathBuf, content: Vec<String>) -> Result<()> {
    conditional_write_vec_string_in_file(&file_path, &content, |_a, _b| true)
}

pub fn truncate_file(path: &PathBuf) -> Result<()> {
    let file = File::create(&path).wrap_err(format!("Error reading file {:?}", &path))?;
    file.set_len(0)
        .wrap_err(format!("Error truncating file {:?}", &path))?;

    Ok(())
}

/// add a Vec<String> in a start to file.
pub fn write_vec_string_in_file_start(file_path: &PathBuf, content: Vec<String>) -> Result<()> {
    if content.len() > 0 {
        let mut new_content = content.clone();
        let original_content = file_to_vec_string(&file_path).wrap_err(FILE_TO_VEC_ERR_MSJ)?;

        new_content.push("\n".to_string()); // add a new line to separate contents

        for line in original_content {
            new_content.push(line);
        }

        truncate_file(&file_path).wrap_err("Error deleting file content")?;

        write_vec_string_in_file(&file_path, new_content)
            .wrap_err(format!("Error writing in file {:?}", &file_path))?;
    }

    Ok(())
}
