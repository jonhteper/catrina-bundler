use super::config::Config;
use super::import::Import;
use super::utils::{
    conditional_write_vec_string_in_file, file_to_string, file_to_vec_string, getwd, truncate_file,
    write_vec_string_in_file, FILE_TO_VEC_ERR_MSJ,
};
use super::ERROR_TO_STR_MSJ;
use eyre::{ContextCompat, Result, WrapErr};
use fs_extra::dir;
use html_minifier::css::minify;
use serde_derive::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Parser {
    config: Config,
    fonts_relation: Vec<RelationCSSFont>,
}

impl Parser {
    pub fn new(config: Config) -> Result<Self> {
        let fonts_relation = Parser::read_fonts_relation(&config)
            .wrap_err("Error reading catrina fonts relation")?;
        Ok(Parser {
            config,
            fonts_relation,
        })
    }

    fn read_fonts_relation(config: &Config) -> Result<Vec<RelationCSSFont>> {
        let mut file_location = PathBuf::from(&config.location_lib);
        file_location.push("css-fonts-relation.json");

        let file = File::open(&file_location)
            .wrap_err(format!("Error reading file {:?}", &file_location))?;

        let data = file_to_string(file).wrap_err("Error in file-to-string conversion")?;

        let fonts_relation = serde_json::from_str(&data).wrap_err("Error deserialize file data")?;

        Ok(fonts_relation)
    }
    /// This function copy fonts files directly in deploy path
    fn copy_fonts(&self, imports: &Vec<Import>) -> Result<()> {
        let mut path_buf: PathBuf;
        for import in imports {
            path_buf = PathBuf::from(&import.path);
            let path = path_buf
                .file_name()
                .context("Error obtaining filename")?
                .to_str()
                .context(ERROR_TO_STR_MSJ)?;
            for f in &self.fonts_relation {
                let mut font = f.clone();
                if font.name == path.to_string() {
                    let mut font_path = PathBuf::from(&self.config.location_lib);
                    font_path.push(PathBuf::from(font.path.clone().replace("./lib/", "")));
                    font.path = font_path.to_str().context(ERROR_TO_STR_MSJ)?.to_string();
                    font.get_font(&self.config)
                        .wrap_err("Error obtaining font files")?;
                }
            } // for font
            path_buf.clear();
        } // for imports
        Ok(())
    } // copy_fonts method

    /// create a Import object from string
    pub fn extract_import(line: &String) -> Import {
        let mut l = line.clone();
        l = l
            .replace("@import ", "")
            .replace(";", "")
            .replace("\"", "")
            .replace("\n", "");
        Import {
            names: vec![],
            path: l,
        }
    }

    /// Search imports in catrina and add necessary code
    pub(crate) fn recursive_imports(
        &self,
        temp_path: &PathBuf,
        imports_list: &Vec<Import>,
    ) -> Result<()> {
        let file_content = file_to_vec_string(&temp_path).wrap_err(FILE_TO_VEC_ERR_MSJ)?;
        let mut skip_lines: Vec<usize> = vec![];
        let mut new_imports: Vec<Import> = vec![];
        let mut next = false;
        for (i, line) in file_content.iter().enumerate() {
            let v = line.contains("/*Colors*/");
            if !v && !next {
                continue;
            }
            if v {
                next = true;
            }

            if line.contains("@import") {
                if !line.contains("core.css") {
                    let import = Parser::extract_import(&line);
                    for im in imports_list {
                        if im.path != import.path {
                            new_imports.push(import.clone());
                        }
                    } // for imports_list
                } // if not core.css
                skip_lines.push(i);
            } // if contains @import
        } // for file content

        new_imports.dedup_by(|a, b| a.path.eq(&b.path));
        skip_lines.dedup();

        Parser::write_file_exclude_lines(&skip_lines, &file_content, &temp_path)
            .wrap_err("Error writing clear imports code")?;

        for import in new_imports {
            self.print_import_file_no_imports(&import, &temp_path)
                .wrap_err("Error writing import file without @import lines")?;
        }

        Ok(())
    }
    pub fn print_imports(&self, imports: &Vec<Import>, temp_file: &PathBuf) -> Result<()> {
        let mut location = PathBuf::from(getwd());
        for import in imports {
            let import_path = import.path.to_string();
            location.push(PathBuf::from(import_path));

            let mut file_content = file_to_vec_string(&location).wrap_err(FILE_TO_VEC_ERR_MSJ)?;
            file_content.push("\n".to_string()); // add a new line to separate files content

            write_vec_string_in_file(temp_file, file_content)
                .wrap_err("Error writing vec in file")?;

            location.clear();
        } // for imports

        self.copy_fonts(imports)
    } //print_imports method

    pub fn print_import_file_no_imports(&self, import: &Import, temp_file: &PathBuf) -> Result<()> {
        let mut path = PathBuf::from("");
        if import.path.contains("../") {
            let raw_path = import.path.replace("../", "");
            path = PathBuf::from(&self.config.location_lib);
            path.push(&raw_path);
        }
        let origin_file_content = file_to_vec_string(&path).wrap_err(FILE_TO_VEC_ERR_MSJ)?;

        conditional_write_vec_string_in_file(&temp_file, &origin_file_content, |_a, b| {
            !b.contains("@import")
        })
        .wrap_err(format!(
            "Error copying file {:?} without @imports",
            &origin_file_content
        ))?;

        Ok(())
    }

    /// write a Vec<String> in a file excluding a specific lines
    pub fn write_file_exclude_lines(
        skip_lines: &Vec<usize>,
        content: &Vec<String>,
        path: &PathBuf,
    ) -> Result<()> {
        if content.len() > 0 {
            truncate_file(&path).wrap_err("Error truncating file")?;

            let mut file = OpenOptions::new()
                .append(true)
                .open(&path)
                .wrap_err("Error opening file to append")?;
            let mut content = content.clone();

            // replace @imports to new line
            for n in skip_lines {
                content[*n] = "\n".to_string();
            }

            for line in content {
                file.write_all(line.as_bytes())
                    .wrap_err("Error writing lines in file")?;
            }
            file.write_all("\n".as_bytes())
                .wrap_err("Error adding final new line")?; // force new line for new appends
        }
        Ok(())
    }

    /// minify a css file
    pub fn minify_file_content(file_path: &PathBuf) -> Result<()> {
        let file = File::open(file_path).wrap_err(format!("Error open file {:?}", &file_path))?;
        let content = file_to_string(file).wrap_err("Error in file-to-string conversion")?;

        let minify_content = minify(&*content).unwrap_or("".to_string());

        let mut file = File::create(&file_path).wrap_err(format!(
            "Error opening file {:?} in write-only mode",
            &file_path
        ))?;
        file.set_len(0).wrap_err("Error deleting file content")?;

        file.write_all(minify_content.as_bytes())
            .wrap_err(format!("Error writing file {:?}", &file_path))?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RelationCSSFont {
    pub name: String,
    pub path: String,
}

impl RelationCSSFont {
    pub fn get_font(&self, config: &Config) -> Result<()> {
        let mut from_paths = Vec::new();
        from_paths.push(&self.path);
        fs_extra::copy_items(&from_paths, &config.deploy_path, &dir::CopyOptions::new()).wrap_err(
            format!(
                "Error copy files {:?} to {}",
                &from_paths, &config.deploy_path
            ),
        )?;

        Ok(())
    }
}
