use crate::catrina::config::{standard_config, Config};
use crate::catrina::js::Parser;
use crate::catrina::lib::StdLib;
use crate::catrina::utils::{file_to_string, getwd};
use crate::catrina::{CONFIG_FILE, DEFAULT_PORT, VERSION_APP};
use eyre::Result;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

extern crate serde;
extern crate serde_json;

pub(crate) struct Project {
    pub config: Config,
    pub name: String,
}

impl Project {
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
        let rand_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect();

        fs::create_dir(&rand_name)?;

        let mut location = getwd();
        location.push(&rand_name);
        Ok(location)
    }

    pub fn build(&self) -> Result<()> {
        /* TODO build project...
            -- make temp dir
            -- bundle js file
                -- read exports file : catrina/exports.js or catrina/catrina.js
                -- bundle all catrina js
                -- create temp file
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
        let temp_location = Project::generate_temp_dir()?;

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
