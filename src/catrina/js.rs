use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use eyre::Result;
use substring::Substring;

use crate::catrina::config::Config;
use crate::catrina::import::Import;
use crate::catrina::utils::file_to_string;

const END_EXPORT: &str = "//@stop";
pub struct Parser {
    directory: Vec<Import>,
}

impl Parser {
    pub fn new(directory: Vec<Import>) -> Parser {
        Parser { directory }
    }

    fn search_in_file(names: &Vec<String>, file_path: &PathBuf) -> Result<String> {
        let file = File::open(file_path)?;
        let data = file_to_string(file)?;
        let mut result = String::new();

        for name in names {
            let content = Parser::search_in_content(&name, &data);
            result.push_str(&content);
        }

        Ok(result)
    }

    fn search_in_content(name: &str, content: &String) -> String {
        let lines = content.lines();
        let mut ev = false;
        let mut result = String::new();

        for line in lines {
            let line_content = line.to_string();
            if (line_content.contains(name) && line_content.contains("export")) || ev {
                //println!("{}", line_content);
                ev = true;
                if line_content.contains(END_EXPORT) {
                    ev = false;
                    continue;
                }
                result.push_str(&line_content);
            }
        }

        result
    }
    // TODO: this func not works, create a buffer and not copy in String...
    pub fn print_imports(&self, imports: Vec<Import>, temp_file_path: &PathBuf) -> Result<()> {
        println!("{:?}", &temp_file_path);
        for import in imports {
            for export in &self.directory {
                if export.path.contains(&import.path) {
                    let parser_result =
                        Parser::search_in_file(&import.names, &PathBuf::from(&export.path))?;

                    let mut file = OpenOptions::new().append(true).open(&temp_file_path)?;

                    file.write_all(parser_result.as_bytes())?;
                }
            }
        }

        Ok(())
    }
}
