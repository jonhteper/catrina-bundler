use crate::args::CatrinaArgs;
use crate::catrina::lib::StdLib;
use crate::catrina::project::{auto_project, Project};
use crate::catrina::utils::{getwd, read_user_response};
use crate::catrina::wizard::run_wizard;
use eyre::Result;
use std::fs;
use std::fs::File;

mod js;
mod lib;
mod project;
mod utils;
mod wizard;
mod config;

const DEFAULT_PORT: &str = ":9095";
const CONFIG_FILE: &str = "catrina.config.json";
pub const VERSION_APP: &str = "v0.0.1";
const START_COMMAND: &str = "init";
const UPDATE_COMMAND: &str = "update";
const RUN_SERVER_COMMAND: &str = "run";
const BUILD_COMMAND: &str = "build";

/// Command catrina init, its works like npm init
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
        auto_project(&project_name.to_string());
        return Ok(());
    }

    println!("Do you want to start the setup wizard?(y/n)");

    let r = read_user_response();
    if r == String::from("y") {
        run_wizard(&project_name.to_string())
    } else {
        auto_project(&project_name.to_string())
    }

    Ok(())
}

fn project_from_location() -> Result<Project> {
    let actual_path = getwd();
    let project_name = actual_path
        .file_name()
        .expect("Error reading current directory ");
    let project_name = project_name.to_str().expect("Error parsing directory name");

    let mut file_path = getwd();
    file_path.push(&CONFIG_FILE);

    let file = File::open(file_path)?;
    let project = Project::from(file, String::from(project_name))?;
    Ok(project)
}

fn catrina_update(flag: bool) -> Result<()> {
    println!("Deprecated!!");
    Ok(())
}

fn catrina_build() -> Result<()> {
    let project = project_from_location()?;
    project.build()?;
    Ok(())
}

pub fn catrina_tool(args: CatrinaArgs) -> Result<()> {
    match &args.action {
        &START_COMMAND => catrina_new(args.skip, args.yarn)?,
        &UPDATE_COMMAND => catrina_update(args.skip)?,
        &BUILD_COMMAND => catrina_build()?,
        _ => {
            println!("{}", &args.action);
        }
    }
    Ok(())
}
