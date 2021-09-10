use crate::args::CatrinaArgs;
use crate::catrina::lib::StdLib;
use crate::catrina::project::{auto_project, Project};
use crate::catrina::utils::{getwd, read_user_response};
use crate::catrina::wizard::run_wizard;
use eyre::Result;
use std::fs::File;

mod config;
mod import;
mod js;
mod lib;
mod project;
mod utils;
mod wizard;

const DEFAULT_PORT: &str = ":9095";
const CONFIG_FILE: &str = "catrina.config.json";
pub const VERSION_APP: &str = "v0.0.2";
const START_COMMAND: &str = "init";
const RUN_SERVER_COMMAND: &str = "run";
const BUILD_COMMAND: &str = "build";

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
        auto_project(project_name);
        return Ok(());
    }

    println!("Do you want to start the setup wizard?(y/n)");

    let r = read_user_response();
    if r == String::from("y") {
        run_wizard(&project_name.to_string())
    } else {
        auto_project(project_name)
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

/// Create a server in project path, using the config port
fn catrina_run_server() -> Result<()> {
    println!("Work in progress...");
    Ok(())
}

/// Run the bundler functions.
fn catrina_build() -> Result<()> {
    let project = project_from_location()?;
    project.build()?;
    Ok(())
}

/// Run the app
pub fn catrina_tool(args: CatrinaArgs) -> Result<()> {
    match &args.action {
        &START_COMMAND => catrina_new(args.skip, args.yarn)?,
        &RUN_SERVER_COMMAND => catrina_run_server()?,
        &BUILD_COMMAND => catrina_build()?,
        _ => {
            println!(
                "No such command {}. Run catrina --help for more info.",
                &args.action
            );
        }
    }
    Ok(())
}
