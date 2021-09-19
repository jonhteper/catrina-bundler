use crate::catrina::utils::{file_to_string, getwd};
use crate::catrina::{CONFIG_FILE, DEFAULT_PORT};
use eyre::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub input_js: String,
    pub input_css: String,
    pub deploy_path: String,
    pub out_js: String,
    pub out_css: String,
    pub server_port: String,
    pub location_lib: String,
    pub module: bool,
    pub minify: bool,
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

    pub fn out_js_path(&self) -> PathBuf {
        let mut path = PathBuf::from(&self.deploy_path);
        path.push(&self.out_js);

        path
    }

    pub fn out_css_path(&self) -> PathBuf {
        let mut path = PathBuf::from(&self.deploy_path);
        path.push(&self.out_css);

        path
    }
}

/// Create a Config object, whit pre-defined values.
///
/// # Arguments
///
/// * `project_name`: This arguments named the output files.
///
/// returns: Config
///
/// # Examples
///
/// ```
/// // In /home/user/My-Project
/// let project_name = "My-Project";
/// let mut project = Project {
///     config: standard_config(project_name),
///     name: project_name.to_string(),
///  };
///
/// let standard_config_example = Config {
///     input_js: "input.js".to_string(),
///     input_css: "input.css".to_string(),
///     deploy_path: "deploy".to_string(),
///     out_js: "My-Project.main.js".to_string(),
///     out_css: "My-Project.styles.css".to_string(),
///     server_port: ":9095".to_string(),
///     location_lib: "/home/user/My-Project/node_modules/catrina".to_string(),
///     module: false,
///     minify: false
/// };
///
/// assert_eq!(project.config, standard_config_example);
/// ```
pub fn standard_config(project_name: &str) -> Config {
    let mut location = getwd();
    location.push("node_modules/catrina");
    let location = location.to_str().expect("");

    Config {
        input_js: "input.js".to_string(),
        input_css: "input.css".to_string(),
        deploy_path: "deploy".to_string(),
        out_js: format!("{}.main.js", project_name),
        out_css: format!("{}.styles.css", project_name),
        server_port: DEFAULT_PORT.to_string(),
        location_lib: location.to_string(),
        module: false,
        minify: false,
    }
}
