use eyre::{ContextCompat, Result, WrapErr};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use substring::Substring;

use crate::catrina::config::Config;
use crate::catrina::import::Import;
use crate::catrina::utils::{file_to_string, write_vec_string_in_file};
use html_minifier::js::minify;
use regex::Regex;

const END_EXPORT: &str = "//@stop";

pub struct Parser {
    directory: Vec<Import>,
}

impl Parser {
    pub fn new(directory: Vec<Import>) -> Parser {
        Parser { directory }
    }

    fn obtain_names(line: &String) -> Result<Vec<String>> {
        let mut names = vec![];
        let names_regex = Regex::new(r"\{.+}").wrap_err("Error in regex expression")?;
        let mut capture_names = names_regex
            .captures(&line)
            .wrap_err("No coincidences in line")?;

        let mut names_str = capture_names
            .get(0)
            .wrap_err("Error getting first regex coincidence value")?
            .as_str();

        let names_list = names_str
            .substring(1, names_str.len() - 1)
            .split(",")
            .collect::<Vec<&str>>();
        for n in names_list {
            names.push(n.trim().to_string());
        }

        Ok(names)
    }

    fn obtain_path(line: &String) -> Result<String> {
        let mut path = String::from("");
        let path_regex = Regex::new("\".+\"").wrap_err("Error in regex expression")?;
        let mut capture_path = path_regex
            .captures(&line)
            .wrap_err("No coincidences in line")?;
        path = capture_path
            .get(0)
            .wrap_err("Error getting first regex coincidence value")?
            .as_str()
            .to_string()
            .replace("\"", "")
            .replace("./", "");
        Ok(path)
    }

    /// create a Import object from import or export javascript file line.
    ///
    /// # Arguments
    ///
    /// * `line`: file line
    /// * `config`: Project.config object
    /// * `canonicalize`: If this is true, the value of Import.path will be an absolute path
    ///
    /// returns: Result<Import, Report>
    ///
    /// # Examples
    ///
    /// ```
    ///     let line = String::from("export {Alert, salert, another,next} from \"./alerts/alert.js\"");
    ///
    ///     let import_result = Parser_js::new_import_by_line(line, &standard_config(""), false)?;
    ///     let import = Import{names:vec!["Alert", "salert", "another", "next"], path: "alerts/alert.js"};
    ///
    ///     assert_eq!(import_result, import);
    /// ```
    pub fn new_import_by_line(
        line: &String,
        config: &Config,
        canonicalize: bool,
    ) -> Result<Import> {
        let mut import = Import::new();
        import.names = Parser::obtain_names(&line).wrap_err("Error getting names from js line")?;
        import.path = Parser::obtain_path(&line).wrap_err("Error getting path from js line")?;

        if canonicalize {
            let path_buf = PathBuf::from(format!("{}/{}", config.location_lib, &import.path))
                .canonicalize()
                .wrap_err("Error canonicalize path")?;

            import.path = path_buf
                .to_str()
                .wrap_err("Error in to-str conversion")?
                .to_string();
        }

        Ok(import)
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

        // Based in https://dev.to/dandyvica/different-ways-of-reading-files-in-rust-2n30
        loop {
            let bytes_read = content.read_line(&mut line)?;
            // EOF: save last file address to restart from this address for next run
            if bytes_read == 0 {
                break;
            }

            if (line.contains(name) && line.contains("export")) || ev {
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
        } //loop

        Ok(content_match)
    }

    /// print imports in a temp file
    pub fn print_imports(&self, imports: Vec<Import>, temp_file_path: &PathBuf) -> Result<()> {
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

    /// minify a javascript file
    pub fn minify_file_content(file_path: &PathBuf) -> Result<()> {
        let file = File::open(file_path)?;
        let content = file_to_string(file)?;

        let minify_content = minify(&*content);

        let mut file = File::create(file_path)?;
        file.set_len(0)?;

        file.write_all(minify_content.as_bytes())?;

        Ok(())
    } // minify_file_content fn
} // Parser
