#![allow(dead_code)]

extern crate serde;
extern crate serde_json;

use catrina_lib::StdLib;
use css::Parser as Parser_css;
use eyre::{ContextCompat, Result, WrapErr};
use js::Parser as Parser_js;
use project::{auto_project, Project};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use utils::{
    file_to_vec_string, getwd, read_user_response, write_vec_string_in_file, FILE_TO_VEC_ERR_MSJ,
};
use wizard::run_wizard;

mod catrina_lib;
mod config;
mod css;
mod import;
mod js;
mod project;
mod utils;
mod wizard;

pub const CONFIG_FILE: &str = "catrina.config.json";

const ERROR_TO_STRING_MSJ: &str = "Error in to-string conversion";
const ERROR_TO_STR_MSJ: &str = "Error in to-str conversion";

/// Create a new project in current path, use npm or yarn depending on the flags
fn catrina_new(skip_flag: bool, yarn_flag: bool) -> Result<()> {
    let actual_path = getwd();
    let project_name = actual_path
        .file_name()
        .expect("Error reading current directory ");
    let project_name = project_name.to_str().expect("Error parsing directory name");

    // install catrina by npm
    let std_lib = StdLib::new(!yarn_flag);
    std_lib.install()?;
    println!("The project has been created successfully!");

    if skip_flag {
        auto_project(project_name).wrap_err("Error initializing default project")?;
        return Ok(());
    }

    println!("Do you want to start the setup wizard?(y/n)");

    let r = read_user_response();
    if r == String::from("y") {
        run_wizard(&project_name.to_string()).wrap_err("Error running wizard")?;
    } else {
        auto_project(project_name).wrap_err("Error initializing default project")?;
    }

    Ok(())
}

/// Create a Project object based in a catrina.config.json file in current file
fn project_from_location() -> Result<Project> {
    let mut file_path = getwd();
    file_path.push(&CONFIG_FILE);

    let file = File::open(file_path)?;
    let project = Project::from(file)?;
    Ok(project)
}

/// Create a minify copy for a css file or javascript file, this file will saved in current directory
/// # Arguments
///
/// * `origin_path`: relative o absolute path to original file
/// * `final_path`: destiny path for final file, can be a dir. The result name looks like this
/// min.originalName.ext.
/// * `delete_original: if is true, delete the original file.
fn catrina_minify(origin_path: &str, final_path: &str, delete_original: bool) -> Result<()> {
    const BAD_EXTENSION: &str = "Only can minify css or javascript files";
    let original_path = PathBuf::from(origin_path);
    let filename = original_path
        .file_name()
        .wrap_err("Error reading filename")?
        .to_str()
        .wrap_err(ERROR_TO_STRING_MSJ)?;
    if original_path.is_dir() {
        println!("{:?} is a directory! {}", &original_path, &BAD_EXTENSION);
        return Ok(());
    }

    let extension = original_path
        .extension()
        .wrap_err("Error reading extension")?
        .to_str()
        .wrap_err(ERROR_TO_STRING_MSJ)?;

    let mut new_path = PathBuf::from(&final_path);
    if final_path == "" {
        new_path = getwd();
    }

    if new_path.is_dir() {
        new_path.push(format!("min.{}", filename));
    }

    fs::copy(&original_path, &new_path).wrap_err(format!(
        "Error copy {:?} to {:?}",
        &original_path, &new_path
    ))?;

    match extension {
        "css" => Parser_css::minify_file_content(&new_path).wrap_err_with(|| {
            fs::remove_file(&new_path).expect("Error removing file");
            format!("Error minifying file {:?}. File is deleted", &new_path)
        })?,
        "js" => Parser_js::minify_file_content(&new_path).wrap_err_with(|| {
            fs::remove_file(&new_path).expect("Error removing file");
            format!("Error minifying file {:?}. File is deleted", &new_path)
        })?,
        _ => {
            println!("{}", &BAD_EXTENSION);
            return Ok(());
        }
    };

    if delete_original {
        fs::remove_file(&original_path).wrap_err("Error deleting original file")?;
    }

    println!("File minified saved in: {:?}", &new_path);
    Ok(())
}

/// Get two filepath, combine this files and save in a location. Optional minify.
/// # Caution
/// If the final file exists, and the flag -m is active, this file will be deleting and a minified
/// file replace it
/*fn catrina_combine(args: &CatrinaArgs) -> Result<()> {
    let first_file = PathBuf::from(&args.filepath_1);
    let second_file = PathBuf::from(&args.filepath_2);
    let final_file = PathBuf::from(&args.filename);

    if first_file.is_dir() || second_file.is_dir() {
        println!("Forbidden operation for directories");
        return Ok(());
    }

    fs::copy(&first_file, &final_file)
        .wrap_err(format!("Error copy {:?} to {:?}", &first_file, &final_file))?;
    let content = file_to_vec_string(&second_file).wrap_err_with(|| {
        fs::remove_file(&final_file).expect("Error deleting final file");
        format!("{}. File {:?} deleted.", FILE_TO_VEC_ERR_MSJ, &second_file)
    })?;
    write_vec_string_in_file(&final_file, content).wrap_err_with(|| {
        fs::remove_file(&final_file).expect("Error deleting final file");
        format!(
            "Error adding file content from {:?} to {:?}. Final file deleted.",
            &second_file, &final_file
        )
    })?;

    if args.minify {
        let file_location = final_file.to_str().wrap_err(ERROR_TO_STRING_MSJ)?;
        let file_parent = final_file
            .parent()
            .wrap_err("Error reading parent")?
            .to_str()
            .wrap_err(ERROR_TO_STRING_MSJ)?;
        catrina_minify(&file_location, &file_parent, true)
            .wrap_err(format!("Error minifying file {:?}", &final_file))?;
    }

    println!("Files combined! Result file saved in {:?}", &final_file);
    Ok(())
}*/

/// Run the bundler functions.
fn catrina_build() -> Result<()> {
    let project = project_from_location()?;
    project.build()?;
    Ok(())
}
