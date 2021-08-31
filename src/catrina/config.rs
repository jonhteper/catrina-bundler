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
    pub fn from_file(file_config: File) -> Result<Config> {
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
