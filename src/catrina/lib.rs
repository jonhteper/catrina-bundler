use crate::catrina::config::Config;
use crate::catrina::import::Import;
use crate::catrina::utils::{bin_dir, getwd};
use crate::catrina::VERSION_APP;
use eyre::Result;
use fs_extra::dir::DirEntryAttr::Path;
use fs_extra::error::ErrorKind::OsString;
use std::ffi::OsStr;
use std::fs;
use std::fs::{create_dir, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;
use std::ptr::replace;
use substring::Substring;

/// catrina standard library
pub struct StdLib {
    npm: bool,
}

impl StdLib {
    pub fn new(default: bool) -> Self {
        StdLib { npm: default }
    }

    ///    install catrina package from npm or yarn
    pub fn install(&self) -> Result<()> {
        if self.npm {
            StdLib::install_by_npm()
                .expect("Error using npm. Please make sure you have the program installed")
        } else {
            StdLib::install_by_yarn()
                .expect("Error using yarn. Please make sure you have the program installed")
        }

        Ok(())
    }

    fn install_by_npm() -> Result<()> {
        /* exec
         * npm init -y
         * npm install --save catrina
         */
        let _output = Command::new("npm").args(&["init", "-y"]).output()?;
        let _output = Command::new("npm")
            .args(&["install", "--save", "catrina"])
            .output()?;

        Ok(())
    }

    fn install_by_yarn() -> Result<()> {
        /* exec
         * yarn init -y
         * yarn add catrina
         */
        let _output = Command::new("yarn").args(&["init", "-y"]).output()?;
        let _output = Command::new("yarn").args(&["add", "catrina"]).output()?;

        Ok(())
    }

    /// Return catrina exports file
    fn exports_file_content(config: &Config) -> Result<File> {
        let mut exports_file_location = PathBuf::from(&config.location_lib);
        exports_file_location.push("catrina.js");

        let file: File;

        match File::open(&exports_file_location) {
            Ok(f) => file = f,
            Err(_e) => {
                println!("No such file catrina.js, trying with exports.js");

                exports_file_location = PathBuf::from(&config.location_lib);
                exports_file_location.push("exports.js");

                file = File::open(&exports_file_location)?;
            }
        };

        Ok(file)
    }

    /// Return a Vec<String> whit lines of export file
    fn exports(config: &Config) -> Result<Vec<String>> {
        let exports_file = StdLib::exports_file_content(config)?;
        let reader = BufReader::new(&exports_file);
        let mut result: Vec<String> = vec![];
        for (_, file_line) in reader.lines().enumerate() {
            result.push(file_line.unwrap_or(String::from("")));
        }

        Ok(result)
    }

    pub fn exports_list(config: &Config) -> Result<Vec<Import>> {
        let exports = StdLib::exports(config)?;
        let mut exports_list: Vec<Import> = vec![];

        for line in exports {
            let export = Import::new_from_line(line, config, true)?;
            exports_list.push(export);
        } // for lines

        Ok(exports_list)
    }

    /// Copy core in a temp file whit name of temp location like this:
    /// `randomName.bundle.js`.
    pub fn bundle_core_js(directory: &Vec<Import>, temp_location: &mut PathBuf) -> Result<()> {
        let filename = temp_location
            .file_name()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("");
        temp_location.push(PathBuf::from(format!("{}.bundle.js", filename)));

        let _temp_file = File::create(&temp_location)?;
        for import in directory {
            if import.path.contains("core.js") {
                fs::copy(&import.path, &mut *temp_location)?;
            }
        }

        Ok(())
    }

    pub fn bundle_all_catrina(directory: Vec<Import>, temp_location: &mut PathBuf) -> Result<()> {
        Ok(())
    }
} //Impl StdLib
