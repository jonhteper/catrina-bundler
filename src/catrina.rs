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

const DEFAULT_PORT: &str = ":9095";
const CONFIG_FILE: &str = "catrina.config.json";
pub const VERSION_APP: &str = "v1.2.0";
const START_COMMAND: &str = "new";
const UPDATE_COMMAND: &str = "update";
const RUN_SERVER_COMMAND: &str = "run";
const BUILD_COMMAND: &str = "build";
const GET_LIB_VERSION_COMMAND: &str = "get";

fn catrina_new(project_name: &str, flag: bool) -> Result<()> {
    if project_name.len() == 0 {
        println!("The project name is necessary. Try with 'catrina new myProject'");
        return Ok(());
    }

    match fs::create_dir(project_name) {
        Ok(_x) => {}
        Err(_e) => {
            println!(
                "the project {} exist, try with a different name",
                project_name
            );
            return Ok(());
        }
    }

    let mut location = getwd();
    location.push(project_name);

    let std_lib = StdLib::new(VERSION_APP, location);
    std_lib.get()?;
    println!("The project has been created successfully!");

    if flag {
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
    if flag {
        let project = project_from_location()?;
        project.update_lib()?;
        return Ok(());
    }

    println!("IMPORTANT! This command delete all additional libraries installed");
    println!("Do you want continue?(y/n)");
    if read_user_response() == "y" {
        let project = project_from_location()?;
        project.update_lib()?;
    }

    Ok(())
}

fn catrina_build() -> Result<()> {
    let project = project_from_location()?;
    project.build()?;
    Ok(())
}

pub fn catrina_tool(args: (&str, &str, bool)) -> Result<()> {
    match &args.0 {
        &START_COMMAND => catrina_new(args.1, args.2)?,
        &UPDATE_COMMAND => catrina_update(args.2)?,
        &BUILD_COMMAND => catrina_build()?,
        _ => {
            println!("{}", &args.0);
        }
    }
    Ok(())
}
