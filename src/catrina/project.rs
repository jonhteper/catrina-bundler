use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::{env, fs};

use eyre::Result;

use crate::catrina::config::{standard_config, Config};
use crate::catrina::import::Import;
use crate::catrina::js::Parser;
use crate::catrina::lib::StdLib;
use crate::catrina::utils::{file_to_vec_string, getwd, write_vec_string_in_file};
use crate::catrina::CONFIG_FILE;

extern crate serde;
extern crate serde_json;

/// Abstraction of a project
pub(crate) struct Project {
    pub config: Config,
}

impl Project {
    /// Project constructor from catrina.config.json file, and name
    pub fn from(file: File) -> Result<Project> {
        let config = Config::from_file(file)?;
        Ok(Project { config })
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

    fn get_imports_js(&self, counter: &mut usize) -> Result<Vec<Import>> {
        let mut imports: Vec<Import> = vec![];
        let input_file = File::open(&self.config.input_js).expect("No such input file");
        let reader = BufReader::new(&input_file);

        for (i, file_line) in reader.lines().enumerate() {
            let line = file_line.unwrap_or("".to_string());
            if line.contains("import") && line.contains("catrina") && !line.contains("core") {
                let import = Import::new_from_line(line, &self.config, false)?;
                imports.push(import);

                *counter = i
            }
        } // loop

        Ok(imports)
    } // get_imports_js method

    fn copy_main_content(&self, line_start: usize, temp_file_path: &PathBuf) -> Result<()> {
        let main_file_content = file_to_vec_string(&PathBuf::from(&self.config.input_js))?;

        let mut raw_lines: Vec<String> = vec![];
        let mut i = 0;

        for line in main_file_content {
            if i > line_start {
                raw_lines.push(line)
            }
            i += 1;
        }

        write_vec_string_in_file(temp_file_path, raw_lines)
    }

    fn remove_js_exports(&self, temp_path: &PathBuf) -> Result<()> {
        if self.config.module {
            return Ok(());
        }

        let file_lines = file_to_vec_string(temp_path)?;
        let mut clear_lines: Vec<String> = vec![];

        if file_lines.len() > 0 {
            for line in file_lines {
                clear_lines.push(line.replace("export ", ""));
            }

            File::create(temp_path)?.set_len(0)?;
            write_vec_string_in_file(temp_path, clear_lines)?;
        }

        Ok(())
    }

    pub fn build(&self) -> Result<()> {
        /* TODO build project...
            -- make temp dir ✅
            -- bundle js file ✅
                -- read exports file : catrina/exports.js or catrina/catrina.js ✅
                -- create temp file and copy catrina core js ✅
                -- read imports ✅
                -- create imports list ✅
                -- copy imports in temp file ✅
                -- remove comments (optional)
                -- replace old bundler for temp file ✅
            -- bundle css file
                -- read imports
                -- create imports list
                -- write imports in temp file
                -- replace old bundler for temp file
                -- copy fonts in deploy dir
            -- remove temp dir ✅
        */

        let mut temp_location = env::temp_dir();

        let directory = StdLib::exports_list(&self.config)?;

        StdLib::bundle_core_js(&directory, &mut temp_location)?;

        let mut line_start: usize = 0;

        let js_imports = self.get_imports_js(&mut line_start)?;

        let parser_js = Parser::new(directory);
        parser_js.print_imports(js_imports, &temp_location)?;

        self.copy_main_content(line_start, &temp_location)?;

        &self.remove_js_exports(&temp_location)?;

        fs::copy(
            &temp_location,
            format!("{}/{}", &self.config.deploy_path, &self.config.out_js),
        )?;

        println!("built!");
        Ok(())
    } // build method
}

pub fn auto_project(project_name: &str) {
    let project = Project {
        config: standard_config(project_name),
    };

    project.start();
}
