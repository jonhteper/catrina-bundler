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

    fn create_environment(&self) {
        fs::create_dir_all(&format!("{}/{}", &self.name, &self.config.deployPath));
        Project::create_input_file(&self.config.inputFileJs, &self.name);
        Project::create_input_file(&self.config.inputFileCSS, &self.name);
        File::create(&format!(
            "{}/{}/{}",
            &self.name, &self.config.deployPath, &self.config.finalFileJS
        ));
        File::create(&format!(
            "{}/{}/{}",
            &self.name, &self.config.deployPath, &self.config.finalFileCSS
        ));
    }

    fn create_input_file(file: &String, project: &String) {
        let mut project_path = PathBuf::from(&project);
        let parent_file = Path::new(&file).parent().unwrap();

        if parent_file.to_str().unwrap().to_string() != String::from("") {
            project_path.push(parent_file);
            fs::create_dir_all(&project_path);

            let mut file_location = PathBuf::from(&project);
            file_location.push(file);
            File::create(file_location);
            return;
        }

        project_path.push(file);
        File::create(project_path);
    }

    fn your_file_config_content(project: &String) {
        let mut data = String::new();
        let reference =
            File::open(&format!("{}/{}", project, CONFIG_FILE)).expect("Error reading config file");
        let mut br = BufReader::new(reference);
        br.read_to_string(&mut data).expect("Error parsing data");
        println!("\nYour project configuration:\n{}", data);
        println!("You can edit this configuration in file {}", CONFIG_FILE);
    }

    pub fn start(&self) {
        &self.config.create_file(&self.name);
        &self.create_environment();
        Project::your_file_config_content(&self.name);
    }

    pub fn update_lib(&self) -> Result<()> {
        let mut actual_path = getwd();
        actual_path.push("lib");
        fs::remove_dir_all(actual_path)?;

        let std_lib = StdLib::new(&self.config.versionLib, getwd());
        std_lib.get()?;
        Ok(())
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
        config: standard_config(),
        name: project_name.to_string(),
    };

    project.start();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub inputFileJs: String,
    pub inputFileCSS: String,
    pub deployPath: String,
    pub finalFileJS: String,
    pub finalFileCSS: String,
    pub serverPort: String,
    pub versionLib: String,
}

impl Config {
    pub fn from_file(mut file_config: File) -> Result<Config> {
        let data = file_to_string(file_config)?;
        let config: Config = serde_json::from_str(&data)?;
        Ok(config)
    }

    pub fn create_file(&self, project: &String) {
        fs::create_dir_all(project);

        let data = serde_json::to_string_pretty(&self).unwrap();
        let file = File::create(&format!("{}/{}", project, CONFIG_FILE))
            .expect("Error creating config file");

        BufWriter::new(file)
            .write_all(data.as_bytes())
            .expect("Error writing config file");
    }
}

pub fn standard_config() -> Config {
    Config {
        inputFileJs: "input.js".to_string(),
        inputFileCSS: "input.css".to_string(),
        deployPath: "./deploy".to_string(),
        finalFileJS: "main.js".to_string(),
        finalFileCSS: "styles.css".to_string(),
        serverPort: DEFAULT_PORT.to_string(),
        versionLib: VERSION_APP.to_string(),
    }
}
