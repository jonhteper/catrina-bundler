use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use eyre::Result;

use crate::catrina::import::Import;
use crate::catrina::utils::write_vec_string_in_file;

const END_EXPORT: &str = "//@stop";

pub struct Parser {
    directory: Vec<Import>,
}

impl Parser {
    pub fn new(directory: Vec<Import>) -> Parser {
        Parser { directory }
    }

    /// Search functions and variables in a file
    fn search_in_file(names: &Vec<String>, file_path: &PathBuf) -> Result<Vec<String>> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(&file);
        let mut content: Vec<String> = vec![];

        for name in names {
            let content_match = Parser::search_name_in_content(name, &mut reader)?;
            if content_match.len() > 0 {
                for line in content_match {
                    content.push(line)
                }
            }
        }

        Ok(content)
    }

    /// search a import in a file content
    fn search_name_in_content(name: &str, content: &mut BufReader<&File>) -> Result<Vec<String>> {
        let mut ev = false;
        let mut line = String::new();
        let mut content_match: Vec<String> = vec![];

        loop {
            match content.read_line(&mut line) {
                Ok(bytes_read) => {
                    // EOF: save last file address to restart from this address for next run
                    if bytes_read == 0 {
                        break;
                    }

                    //println!("{}", line);
                    if (line.contains(name) && line.contains("export")) || ev {
                        //println!("{}", line_content);
                        ev = true;
                        if line.contains(END_EXPORT) {
                            ev = false;
                            line.clear();
                            continue;
                        }

                        content_match.push(line.clone())
                    }

                    // do not accumulate data
                    line.clear();
                }
                Err(err) => {
                    return Err(eyre::Report::from(err));
                }
            };
        } //loop

        Ok(content_match)
    }

    /// print imports in a temp file
    pub fn print_imports(&self, imports: Vec<Import>, temp_file_path: &PathBuf) -> Result<()> {
        //println!("{:?}", &temp_file_path);
        for import in imports {
            for export in &self.directory {
                if export.path.contains(&import.path) {
                    let parser_result =
                        Parser::search_in_file(&import.names, &PathBuf::from(&export.path))?;

                    write_vec_string_in_file(temp_file_path, parser_result)?;
                } // if contains import.path
            } // for directory
        } // for imports

        Ok(())
    } // print_imports method
} // Parser
