use eyre::{ContextCompat, Result, WrapErr};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use substring::Substring;

use crate::catrina::config::Config;
use crate::catrina::import::Import;
use crate::catrina::utils::{file_to_string, file_to_vec_string, write_vec_string_in_file};
use html_minifier::js::minify;
use regex::Regex;

const END_EXPORT: &str = "//@stop";

pub struct Parser {
    directory: Vec<Import>,
    config: Config,
}

impl Parser {
    pub fn new(directory: Vec<Import>, config: Config) -> Parser {
        Parser { directory, config }
    }

    pub fn obtain_names(line: &String) -> Result<Vec<String>> {
        let mut names = vec![];
        let names_regex = Regex::new(r"\{.+}").wrap_err("Error in regex expression")?;
        let capture_names = names_regex
            .captures(&line)
            .wrap_err("No coincidences in line")?;

        let names_str = capture_names
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

    pub fn obtain_path(line: &String) -> Result<String> {
        let path_regex = Regex::new("\".+\"").wrap_err("Error in regex expression")?;
        let capture_path = path_regex
            .captures(&line)
            .wrap_err("No coincidences in line")?;
        let path = capture_path
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
    fn search_in_file(&self, name: &String, file_path: &PathBuf) -> Result<Vec<String>> {
        let file = File::open(&file_path).wrap_err("Error opening file")?;
        let mut reader = BufReader::new(&file);
        let mut content: Vec<String> = vec![];

        //for name in names {
        let content_match = Parser::search_name_in_content(name, &mut reader)?;
        if content_match.len() > 0 {
            for line in content_match {
                content.push(line)
            }
        }
        //}

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

    fn search_path_in_directory(&self, name: &String) -> Option<PathBuf> {
        for import in &self.directory {
            for n in &import.names {
                if name.to_string() == n.to_string() {
                    return Some(import.path_buf());
                }
            }
        }

        None
    }

    fn recursive_imports(&self, imports_list: &Import) -> Result<Import> {
        let mut imports = imports_list.clone();
        let mut names = vec![];

        for n in &imports_list.names {
            let name = n.clone();
            let path = self
                .search_path_in_directory(&name)
                .wrap_err("Import isn't in Catrina")?;

            let content_file =
                file_to_vec_string(&path).wrap_err("Error in file-to-vec conversion")?;

            for line in content_file {
                if line.contains("import") && !line.contains("core.js") {
                    names = Parser::obtain_names(&line)
                        .wrap_err(format!("Error creating import from line {}", &line))?;

                    for name in &names {
                        imports.names.push(name.clone())
                    }

                    names.clear();
                }
            } // for content file
        } // for names

        imports.names.dedup();

        Ok(imports)
    } // recursive imports method

    /// print imports in a temp file
    pub fn print_imports(&self, imports: Import, temp_file_path: &PathBuf) -> Result<()> {
        // TODO print recursive imports
        let imports = self
            .recursive_imports(&imports)
            .wrap_err("Error obtaining internal dependencies")?;
        for name in imports.names {
            let path = self
                .search_path_in_directory(&name)
                .wrap_err("Import isn't in Catrina")?;

            //for export in &self.directory {
            //  if export.path.contains(&import.path) {
            let parser_result = self.search_in_file(&name, &path)?;

            write_vec_string_in_file(temp_file_path, parser_result)?;
            //} // if contains import.path
            //} // for directory
        } // for names

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
