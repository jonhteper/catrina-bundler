use crate::catrina::config::standard_config;
use crate::catrina::project::Project;
use crate::catrina::utils::read_user_response;

pub fn run_wizard(project_name: &String) {
    const EXIT_MSJ: &str = "(type 'exit' to close)";
    const EXIT_ORDER: &str = "exit";
    let mut project = Project {
        config: standard_config(project_name),
        name: project_name.to_string(),
    };

    let standard_config = standard_config(project_name);

    println!("Set deploy path:{}", EXIT_MSJ);
    project.config.deploy_path = read_user_response();
    if project.config.deploy_path == EXIT_ORDER {
        project.config.deploy_path = standard_config.deploy_path;
        project.start();
        return;
    }

    println!("Set final javascript filename:{}", EXIT_MSJ);
    project.config.out_js = read_user_response();
    if project.config.out_js == EXIT_ORDER {
        project.config.out_js = standard_config.out_js;
        project.start();
        return;
    }

    println!("Set final css filename:{}", EXIT_MSJ);
    project.config.out_css = read_user_response();
    if project.config.out_css == EXIT_ORDER {
        project.config.out_css = standard_config.out_css;
        project.start();
        return;
    }

    println!("Set path of input javascript filename:{}", EXIT_MSJ);
    project.config.input_js = read_user_response();
    if project.config.input_js == EXIT_ORDER {
        project.config.input_js = standard_config.input_js;
        project.start();
        return;
    }

    println!("Set path of input css filename:{}", EXIT_MSJ);
    project.config.input_css = read_user_response();
    if project.config.input_css == EXIT_ORDER {
        project.config.input_css = standard_config.input_css;
        project.start();
        return;
    }

    println!("Set port of trial server:{}", EXIT_MSJ);
    project.config.server_port = read_user_response();
    if project.config.server_port == EXIT_ORDER {
        project.config.server_port = standard_config.server_port;
    }

    // TODO add lib location and module

    project.start();
}
