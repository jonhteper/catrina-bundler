use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use eyre::Result;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::catrina::config::{standard_config, Config};
use crate::catrina::import::Import;
use crate::catrina::js::Parser;
use crate::catrina::lib::StdLib;
use crate::catrina::utils::{file_to_string, getwd, random_name};
use crate::catrina::{CONFIG_FILE, DEFAULT_PORT, VERSION_APP};

extern crate serde;
extern crate serde_json;

/// Abstraction of a project
pub(crate) struct Project {
    pub config: Config,
    pub name: String,
}

impl Project {
    /// Project constructor from catrina.config.json file, and name
    pub fn from(file: File, name: String) -> Result<Project> {
        let config = Config::from_file(file)?;
        Ok(Project { config, name })
    }

    fn create_environment(&self) -> Result<()> {
        fs::create_dir_all(&self.config.deploy_path)?;
        Project::create_input_file(&self.config.input_js)?;
        Project::create_input_file(&self.config.input_css)?;
        File::create(&format!(
            "{}/{}",
            &self.config.deploy_path, &self.config.out_js
        ))?;
        File::create(&format!(
            "{}/{}",
            &self.config.deploy_path, &self.config.out_css
        ))?;

        Ok(())
    }

    fn create_input_file(file: &String) -> Result<()> {
        let mut project_path = getwd();
        let parent_file = Path::new(&file).parent().unwrap();

        if parent_file.to_str().unwrap().to_string() != String::from("") {
            project_path.push(parent_file);
            fs::create_dir_all(&project_path)?;

            let mut file_location = getwd();
            file_location.push(file);
            File::create(file_location)?;
            return Ok(());
        }

        project_path.push(file);
        File::create(project_path)?;
        Ok(())
    }

    fn your_file_config_content() {
        let mut data = String::new();
        let reference = File::open(CONFIG_FILE).expect("Error reading config file");
        let mut br = BufReader::new(reference);
        br.read_to_string(&mut data).expect("Error parsing data");
        println!("\nYour project configuration:\n{}", data);
        println!("You can edit this configuration in file {}", CONFIG_FILE);
    }

    pub fn start(&self) {
        &self.config.create_file();
        &self.create_environment();
        Project::your_file_config_content();
    }

    fn generate_temp_dir() -> Result<PathBuf> {
        let rand_name = random_name(15);
        fs::create_dir(&rand_name)?;

        let mut location = getwd();
        location.push(&rand_name);
        Ok(location)
    }

    pub fn build(&self) -> Result<()> {
        /* TODO build project...
            -- make temp dir ✅
            -- bundle js file
                -- read exports file : catrina/exports.js or catrina/catrina.js ✅
                -- create temp file and copy catrina core js ✅
                -- read imports
                -- create imports list
                -- copy core in temp file
                -- copy imports in temp file
                -- replace old bundler for temp file
            -- bundle css file
                -- read imports
                -- create imports list
                -- write imports in temp file
                -- replace old bundler for temp file
                -- copy fonts in deploy dir
            -- remove temp dir
        */
        let mut temp_location = Project::generate_temp_dir()?;

        let mut directory: Vec<Import> = vec![];
        match StdLib::exports_list(&self.config) {
            Ok(d) => directory = d,
            Err(e) => {
                fs::remove_dir_all(&temp_location)?;
                println!("Error: {}", e);
            }
        }

        match StdLib::bundle_core_js(&directory, &mut temp_location) {
            Err(e) => {
                fs::remove_dir_all(&temp_location)?;
                println!("Error: {}", e);
            }

            _ => {}
        }

        //fs::remove_dir_all(temp_location)?;
        Ok(())
    }
}

pub fn auto_project(project_name: &String) {
    let project = Project {
        config: standard_config(project_name),
        name: project_name.to_string(),
    };

    project.start();
}
