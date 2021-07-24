use crate::catrina::project::{standard_config, Project};
use crate::catrina::utils::read_user_response;

pub fn run_wizard(project_name: &String) {
    const EXIT_MSJ: &str = "(type 'exit' to close)";
    const EXIT_ORDER: &str = "exit";
    let mut project = Project {
        config: standard_config(),
        name: project_name.to_string(),
    };

    let standard_config = standard_config();

    println!("Set deploy path:{}", EXIT_MSJ);
    project.config.deployPath = read_user_response();
    if project.config.deployPath == EXIT_ORDER {
        project.config.deployPath = standard_config.deployPath;
        project.start();
        return;
    }

    println!("Set final javascript filename:{}", EXIT_MSJ);
    project.config.finalFileJS = read_user_response();
    if project.config.finalFileJS == EXIT_ORDER {
        project.config.finalFileJS = standard_config.finalFileJS;
        project.start();
        return;
    }

    println!("Set final css filename:{}", EXIT_MSJ);
    project.config.finalFileCSS = read_user_response();
    if project.config.finalFileCSS == EXIT_ORDER {
        project.config.finalFileCSS = standard_config.finalFileCSS;
        project.start();
        return;
    }

    println!("Set path of input javascript filename:{}", EXIT_MSJ);
    project.config.inputFileJs = read_user_response();
    if project.config.inputFileJs == EXIT_ORDER {
        project.config.inputFileJs = standard_config.inputFileJs;
        project.start();
        return;
    }

    println!("Set path of input css filename:{}", EXIT_MSJ);
    project.config.inputFileCSS = read_user_response();
    if project.config.inputFileCSS == EXIT_ORDER {
        project.config.inputFileCSS = standard_config.inputFileCSS;
        project.start();
        return;
    }

    println!("Set port of trial server:{}", EXIT_MSJ);
    project.config.serverPort = read_user_response();
    if project.config.serverPort == EXIT_ORDER {
        project.config.serverPort = standard_config.serverPort;
    }

    project.start();
}
