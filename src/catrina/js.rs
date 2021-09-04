use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use eyre::Result;
use substring::Substring;

use crate::catrina::config::Config;
use crate::catrina::utils::file_to_string;

const END_EXPORT: &str = "//@stop";
pub struct Parser {
    files: Vec<String>,
}

impl Parser {
    pub fn new(files: Vec<String>) -> Parser {
        Parser { files }
    }

    fn search_in_file(names: Vec<&str>, file_path: &PathBuf) -> Result<Option<String>> {
        let file = File::open(file_path)?;
        let data = file_to_string(file)?;
        let mut result = String::new();

        for name in names {
            match Parser::search_in_content(&name, &data) {
                Some(x) => result.push_str(&x),
                None => continue,
            }
        }

        Ok(Some(result))
    }

    fn search_in_content(name: &str, content: &String) -> Option<String> {
        let lines = content.lines();
        let mut ev = false;
        let mut result = String::new();

        for line in lines {
            let line_content = line.to_string();
            if (line_content.contains(name) && line_content.contains("export")) || ev {
                println!("{}", line_content);
                ev = true;
                if line_content.contains(END_EXPORT) {
                    ev = false;
                    continue;
                }
                result.push_str(&line_content);
            }
        }

        Some(result)
    }

    pub fn get_imports_from_file(&self, name: &str) -> Result<String> {
        // TEST... TODO read imports, create list and use Parser::search_in_file
        let path = PathBuf::from("./lib/alerts/alert.js");
        let names = vec!["salert"];
        let parser_result = Parser::search_in_file(names, &path)?;
        return match parser_result {
            Some(x) => Ok(x),
            None => Ok(String::from("")),
        };
    }
}
