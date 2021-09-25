use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::{env, fs};

use eyre::{Result, WrapErr};

use crate::catrina::config::{standard_config, Config};
use crate::catrina::css::Parser as Parser_css;
use crate::catrina::import::Import;
use crate::catrina::js::Parser;
use crate::catrina::lib::StdLib;
use crate::catrina::utils::{
    file_to_vec_string, getwd, random_name, write_vec_string_in_file,
    write_vec_string_in_file_start, FILE_TO_VEC_ERR_MSJ,
};
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

    fn create_deploy_structure(&self) -> Result<()> {
        fs::create_dir_all(&self.config.deploy_path)?;
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

    fn create_environment(&self) -> Result<()> {
        Project::create_input_file(&self.config.input_js)
            .wrap_err("Error creating input js file")?;
        Project::create_input_file(&self.config.input_css)
            .wrap_err("Error creating input css file")?;
        self.create_deploy_structure()
            .wrap_err("Error creating deploy structure")?;

        Ok(())
    }

    fn your_file_config_content() -> Result<()> {
        let mut data = String::new();
        let reference = File::open(CONFIG_FILE).wrap_err("Error reading config file")?;
        let mut br = BufReader::new(reference);
        br.read_to_string(&mut data)
            .wrap_err("Error parsing data")?;
        println!("\nYour project configuration:\n{}", data);
        println!("You can edit this configuration in file {}", CONFIG_FILE);
        Ok(())
    }

    pub fn start(&self) -> Result<()> {
        self.config
            .create_file()
            .wrap_err("Error creating config file")?;
        self.create_environment()
            .wrap_err("Error creating project structure")?;

        Project::your_file_config_content().wrap_err("Error in final ")?;
        Ok(())
    }

    fn get_imports_js(&self, counter: &mut usize) -> Result<Vec<Import>> {
        let mut imports: Vec<Import> = vec![];
        let input_file = File::open(&self.config.input_js).wrap_err("No such input file")?;
        let reader = BufReader::new(&input_file);

        for (i, file_line) in reader.lines().enumerate() {
            let line = file_line.unwrap_or("".to_string());
            if line.contains("import") && line.contains("catrina") && !line.contains("core") {
                let import = Parser::new_import_by_line(&line, &self.config, false)
                    .wrap_err(format!("Error obtaining export in line {}", &line))?;
                imports.push(import);

                *counter = i
            }
        } // loop

        Ok(imports)
    } // get_imports_js method

    fn get_imports_css(&self, counter: &mut usize) -> Result<Vec<Import>> {
        let mut imports: Vec<Import> = vec![];
        let input_file = File::open(&self.config.input_css)?;
        let reader = BufReader::new(&input_file);

        for (i, file_line) in reader.lines().enumerate() {
            let import_path = file_line?;
            if import_path.contains("@import")
                && import_path.contains("catrina/")
                && !import_path.contains("core.css")
            {
                imports.push(Parser_css::extract_import(&import_path));
                *counter = i
            } // if
        } //for

        Ok(imports)
    } // get_imports_css method

    fn copy_all_content(
        &self,
        from_line_start: usize,
        from_file_path: &PathBuf,
        to_file_path: &PathBuf,
    ) -> Result<()> {
        let from_file_content = file_to_vec_string(from_file_path).wrap_err(FILE_TO_VEC_ERR_MSJ)?;

        let mut raw_lines: Vec<String> = vec![];
        let mut i = 0;

        for line in from_file_content {
            if i > from_line_start {
                raw_lines.push(line)
            }
            i += 1;
        }

        write_vec_string_in_file(to_file_path, raw_lines)
    }

    fn copy_all_content_in_start(
        &self,
        from_line_start: usize,
        from_file_path: &PathBuf,
        to_file_path: &PathBuf,
    ) -> Result<()> {
        let from_file_content = file_to_vec_string(from_file_path).wrap_err(FILE_TO_VEC_ERR_MSJ)?;

        let mut raw_lines: Vec<String> = vec![];
        let mut i = 0;

        for line in from_file_content {
            if i > from_line_start {
                raw_lines.push(line)
            }
            i += 1;
        }

        write_vec_string_in_file_start(to_file_path, raw_lines)
    }

    fn remove_js_exports(&self, temp_path: &PathBuf) -> Result<()> {
        if self.config.module {
            return Ok(());
        }

        let file_lines = file_to_vec_string(temp_path)?;
        let mut clear_lines: Vec<String> = vec![];

        if file_lines.len() > 0 {
            for line in file_lines {
                if line.contains("import") {
                    continue;
                }
                clear_lines.push(line.replace("export ", ""));
            }

            File::create(temp_path)?.set_len(0)?;
            write_vec_string_in_file(temp_path, clear_lines)?;
        }

        Ok(())
    }

    /// bundle js file
    fn build_js(&self) -> Result<()> {
        let mut temp_location = env::temp_dir();

        let directory =
            StdLib::exports_js_list(&self.config).wrap_err("Error getting exports list")?;

        StdLib::bundle_core_js(&directory, &mut temp_location).wrap_err_with(|| {
            fs::remove_file(&temp_location).expect("Error deleting temporal file");
            "Error bundle core js"
        })?;

        let mut line_start: usize = 0;

        let js_imports = self.get_imports_js(&mut line_start).wrap_err_with(|| {
            fs::remove_file(&temp_location).expect("Error deleting temporal file");
            "Error getting imports list"
        })?;

        let parser_js = Parser::new(directory);
        parser_js
            .print_imports(js_imports, &temp_location)
            .wrap_err_with(|| {
                fs::remove_file(&temp_location).expect("Error deleting temporal file");
                "Error printing imports in a temp file"
            })?;

        self.copy_all_content_in_start(
            line_start,
            &PathBuf::from(&self.config.input_js),
            &temp_location,
        )
        .wrap_err_with(|| {
            fs::remove_file(&temp_location).expect("Error deleting temporal file");
            "Error coping main js content in a temp file"
        })?;

        self.remove_js_exports(&temp_location).wrap_err_with(|| {
            fs::remove_file(&temp_location).expect("Error deleting temporal file");
            "Error deleting unnecessary code"
        })?;

        if self.config.minify {
            Parser::minify_file_content(&temp_location).wrap_err_with(|| {
                fs::remove_file(&temp_location).expect("Error deleting temporal file");
                "Error minifying code"
            })?;
        }

        fs::copy(
            &temp_location,
            format!("{}/{}", &self.config.deploy_path, &self.config.out_js),
        )
        .wrap_err_with(|| {
            fs::remove_file(&temp_location).expect("Error deleting temporal file");
            "Error replacing old bundle file"
        })?;

        fs::remove_file(&temp_location).wrap_err("Error deleting js temp file")?;
        Ok(())
    }

    /// Search imports in catrina and add necessary code
    fn css_recursive_imports_css(
        &self,
        temp_path: &PathBuf,
        imports_list: &Vec<Import>,
        parser: &Parser_css,
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
                    let import = Parser_css::extract_import(&line);
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

        Parser_css::write_file_exclude_lines(&skip_lines, &file_content, &temp_path)
            .wrap_err("Error writing clear imports code")?;

        for import in new_imports {
            parser
                .print_import_file_no_imports(&import, &temp_path)
                .wrap_err("Error writing import file without @import lines")?;
        }

        Ok(())
    }

    /// bundle css file and copy fonts
    fn build_css(&self) -> Result<()> {
        let mut temp_location = env::temp_dir();

        StdLib::bundle_core_css(&self.config, &mut temp_location).wrap_err_with(|| {
            fs::remove_file(&temp_location).expect("Error deleting temporal file");
            "Error bundling core css"
        })?;

        let mut line_start: usize = 0;
        let imports = self.get_imports_css(&mut line_start).wrap_err_with(|| {
            fs::remove_file(&temp_location).expect("Error deleting temporal file");
            "Error getting css imports"
        })?;

        let parser_css = Parser_css::new(self.config.clone())?;

        parser_css
            .print_imports(&imports, &temp_location)
            .wrap_err_with(|| {
                fs::remove_file(&temp_location).expect("Error deleting temporal file");
                "Error printing css imports"
            })?;

        self.copy_all_content_in_start(
            line_start,
            &PathBuf::from(&self.config.input_css),
            &temp_location,
        )
        .wrap_err_with(|| {
            fs::remove_file(&temp_location).expect("Error deleting temporal file");
            "Error coping main js content in a temp file"
        })?;

        self.css_recursive_imports_css(&temp_location, &imports, &parser_css)
            .wrap_err_with(|| {
                fs::remove_file(&temp_location).expect("Error deleting temporal file");
                "Error with recursive catrina imports"
            })?;

        if self.config.minify {
            Parser_css::minify_file_content(&temp_location).wrap_err_with(|| {
                fs::remove_file(&temp_location).expect("Error deleting temporal file");
                "Error minifying code"
            })?;
        }

        fs::copy(&temp_location, &self.config.out_css_path_buf()).wrap_err_with(|| {
            fs::remove_file(&temp_location).expect("Error deleting temporal file");
            "Error replacing old bundle file"
        })?;

        fs::remove_file(&temp_location).wrap_err("Error deleting css temp file")?;
        Ok(())
    }

    fn create_backup(&self) -> Result<PathBuf> {
        let mut deploy_location = PathBuf::from(&self.config.deploy_path.replace("./", ""));
        if deploy_location.is_relative() {
            let relative_path = deploy_location.clone();
            deploy_location = getwd();
            deploy_location.push(relative_path);
        }
        let random_name = random_name(15);
        let mut to = PathBuf::from(deploy_location.clone().parent().unwrap());

        to.push(format!(".{}", &random_name));

        fs::rename(&deploy_location, &to).wrap_err(format!(
            "Error moving files from {:?} to {:?}",
            &deploy_location, &to
        ))?;

        self.create_deploy_structure()
            .wrap_err("Error recreating deploy path")?;

        Ok(PathBuf::from(to))
    }

    fn recovery_backup(&self, location: &PathBuf) -> Result<()> {
        let to = PathBuf::from(&self.config.deploy_path);
        fs::remove_dir_all(&to).wrap_err(format!("Error removing files in {:?}", &to))?;
        fs::create_dir_all(&to).wrap_err(format!("Error creating dirs in {:?}", &to))?;
        fs::rename(&location, &to).wrap_err(format!(
            "Error moving files from {:?} to {:?}",
            &location, &to
        ))?;

        println!("Error with build command, backup recovered");
        Ok(())
    }

    pub fn build(&self) -> Result<()> {
        let backup_path = self.create_backup().wrap_err("Error creating backup")?;

        self.build_js().wrap_err_with(|| {
            self.recovery_backup(&backup_path)
                .expect("Error in recovery");
            "Error bundle js file"
        })?;

        self.build_css().wrap_err_with(|| {
            self.recovery_backup(&backup_path)
                .expect("Error in recovery");
            "Error bundle css file"
        })?;

        fs::remove_dir_all(&backup_path).wrap_err("Error deleting recovery")?;

        println!("built!");
        Ok(())
    } // build method
}

pub fn auto_project(project_name: &str) -> Result<()> {
    let project = Project {
        config: standard_config(project_name),
    };

    project.start().wrap_err("Error starting project")?;

    Ok(())
}
