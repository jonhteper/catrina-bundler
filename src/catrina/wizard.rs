use crate::catrina::config::standard_config;
use crate::catrina::project::Project;
use crate::catrina::utils::read_user_response;

pub fn run_wizard(project_name: &String) {
    const EXIT_MSJ: &str = "(type 'exit' to close)";
    const EXIT_ORDER: &str = "exit";
    let mut project = Project {
        config: standard_config(project_name),
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

    println!("Set path of input javascript file:{}", EXIT_MSJ);
    project.config.input_js = read_user_response();
    if project.config.input_js == EXIT_ORDER {
        project.config.input_js = standard_config.input_js;
        project.start();
        return;
    }

    println!("Set path of input css file:{}", EXIT_MSJ);
    project.config.input_css = read_user_response();
    if project.config.input_css == EXIT_ORDER {
        project.config.input_css = standard_config.input_css;
        project.start();
        return;
    }

    println!(
        "Set catrina absolute location or set 'default': {}",
        EXIT_MSJ
    );
    project.config.location_lib = read_user_response();
    if project.config.location_lib == EXIT_ORDER
        || project.config.location_lib == "default".to_string()
    {
        project.config.location_lib = standard_config.location_lib;

        if project.config.location_lib == EXIT_ORDER {
            project.start();
            return;
        }
    }

    println!("Final javascript file will be a module?:(y/n/exit)");
    let bool_response = read_user_response();
    if bool_response == EXIT_ORDER {
        project.config.module = standard_config.module;
        project.start();
        return;
    }
    project.config.module = bool_response == "y".to_string();

    println!("Will the final javascript file be minified?:(y/n/exit)");
    let bool_response = read_user_response();
    if bool_response == EXIT_ORDER {
        project.config.minify = standard_config.minify;
        project.start();
        return;
    }
    project.config.minify = bool_response == "y".to_string();

    println!("Set port of trial server:{}", EXIT_MSJ);
    project.config.server_port = read_user_response();
    if project.config.server_port == EXIT_ORDER {
        project.config.server_port = standard_config.server_port;
    }

    project.start();
}
