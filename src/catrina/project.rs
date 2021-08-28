use crate::catrina::js::Parser;
use crate::catrina::lib::StdLib;
use crate::catrina::utils::{file_to_string, getwd};
use crate::catrina::{CONFIG_FILE, DEFAULT_PORT, VERSION_APP};
use eyre::Result;
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

    fn your_file_config_content(project: &String) {
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
        Project::your_file_config_content(&self.name);
    }

    pub fn build(&self) -> Result<()> {
        // TODO build project...
        let mut files: Vec<String> = vec![];
        files.push(String::from("file1"));
        let parser = Parser::new(files);
        let result = parser.get_imports_from_file("name")?;
        println!("{}", result);
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub input_js: String,
    pub input_css: String,
    pub deploy_path: String,
    pub out_js: String,
    pub out_css: String,
    pub server_port: String,
    pub location_lib: String,
    pub module: bool,
}

impl Config {
    pub fn from_file(mut file_config: File) -> Result<Config> {
        let data = file_to_string(file_config)?;
        let config: Config = serde_json::from_str(&data)?;
        Ok(config)
    }

    pub fn create_file(&self) {
        let data = serde_json::to_string_pretty(&self).unwrap();
        let file = File::create(CONFIG_FILE).expect("Error creating config file");

        BufWriter::new(file)
            .write_all(data.as_bytes())
            .expect("Error writing config file");
    }
}

pub fn standard_config(project_name: &str) -> Config {
    Config {
        input_js: "input.js".to_string(),
        input_css: "input.css".to_string(),
        deploy_path: "./deploy".to_string(),
        out_js: format!("{}.main.js", project_name),
        out_css: format!("{}.styles.css", project_name),
        server_port: DEFAULT_PORT.to_string(),
        location_lib: "node_modules/catrina".to_string(),
        module: false,
    }
}
