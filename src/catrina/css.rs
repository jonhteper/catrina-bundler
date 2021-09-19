extern crate serde;
extern crate serde_json;

use crate::catrina::config::Config;
use crate::catrina::import::Import;
use crate::catrina::utils::{
    conditional_write_vec_string_in_file, file_to_string, file_to_vec_string, getwd, truncate_file,
    write_vec_string_in_file, FILE_TO_VEC_ERR_MSJ,
};
use eyre::{Result, WrapErr};
use fs_extra::dir;
use fs_extra::error::ErrorKind::OsString;
use html_minifier::css::minify;
use std::ffi::OsStr;
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
        let fonts_relation =
            Parser::read_fonts_relation(&config).context("Error reading catrina fonts relation")?;
        Ok(Parser {
            config,
            fonts_relation,
        })
    }

    fn read_fonts_relation(config: &Config) -> Result<Vec<RelationCSSFont>> {
        let mut file_location = PathBuf::from(&config.location_lib);
        file_location.push("css-fonts-relation.json");
        let mut fonts_relation: Vec<RelationCSSFont> = vec![];

        let file = File::open(&file_location)
            .context(format!("Error reading file {:?}", &file_location))?;

        let data = file_to_string(file).context("Error in file-to-string conversion")?;

        fonts_relation = serde_json::from_str(&data).context("Error deserialize file data")?;

        Ok(fonts_relation)
    }
    /// This function copy fonts files directly in deploy path
    fn copy_fonts(&self, imports: &Vec<Import>) -> Result<()> {
        let mut path_buf: PathBuf;
        for import in imports {
            path_buf = PathBuf::from(&import.path);
            let path = path_buf
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            for font in &self.fonts_relation {
                if font.name == path.to_string() {
                    let mut font_path = PathBuf::from(&self.config.location_lib);
                    font_path.push(PathBuf::from(font.path.clone().replace("./lib/", "")));
                    let mut from = Vec::new();
                    from.push(font_path);
                    fs_extra::copy_items(&from, &self.config.deploy_path, &dir::CopyOptions::new())
                        .context(format!("Error copy {} font files", &font.name));
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

    pub fn print_imports(&self, imports: &Vec<Import>, temp_file: &PathBuf) -> Result<()> {
        let mut location = PathBuf::new();
        for import in imports {
            location = PathBuf::from(getwd());
            let import_path = import.path.to_string();
            location.push(PathBuf::from(import_path));

            let mut file_content = file_to_vec_string(&location).context(FILE_TO_VEC_ERR_MSJ)?;
            file_content.push("\n".to_string()); // add a new line to separate files content

            write_vec_string_in_file(temp_file, file_content)
                .context("Error writing vec in file")?;

            location.clear();
        } // for imports

        self.copy_fonts(imports)
    } //print_imports method

    pub fn print_import_file_no_imports(&self, import: &Import, temp_file: &PathBuf) -> Result<()> {
        let mut path = PathBuf::from("");
        let mut raw_path = import.path.clone();
        if import.path.contains("../") {
            raw_path = import.path.replace("../", "");
            path = PathBuf::from(&self.config.location_lib);
            path.push(&raw_path);
        }
        let origin_file_content = file_to_vec_string(&path).context(FILE_TO_VEC_ERR_MSJ)?;

        conditional_write_vec_string_in_file(&temp_file, &origin_file_content, |a, b| {
            !b.contains("@import")
        });

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
        let mut file =
            File::open(file_path).context(format!("Error open file {:?}", &file_path))?;
        let content = file_to_string(file).context("Error in file-to-string conversion")?;

        let minify_content = minify(&*content).unwrap_or("".to_string());

        let mut file = File::create(&file_path).context(format!(
            "Error opening file {:?} in write-only mode",
            &file_path
        ))?;
        file.set_len(0).context("Error deleting file content")?;

        file.write_all(minify_content.as_bytes())
            .context(format!("Error writing file {:?}", &file_path))?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RelationCSSFont {
    pub name: String,
    pub path: String,
}

impl RelationCSSFont {
    pub fn get_font(&self, config: &Config) -> Result<()> {
        let options = dir::CopyOptions::new();
        let mut from_paths = Vec::new();
        from_paths.push(&self.path);

        fs_extra::copy_items(&from_paths, &config.deploy_path, &options)?;

        Ok(())
    }
}
