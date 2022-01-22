use crate::args::CatrinaArgs;
use clap::{App, Arg, ArgMatches, SubCommand};

mod args;
mod catrina;

#[macro_use]
extern crate serde_derive;
extern crate clap;

const VERSION_APP: &str = env!("CARGO_PKG_VERSION");

fn app_matches() -> ArgMatches<'static> {
    let app = App::new("Catrina")
        .version(VERSION_APP)
        .author("jonhteper <jonhteper@triamseletea.com>")
        .about("A mini bundler for npm catrina package")
        .subcommand(
            SubCommand::with_name("init")
                .about("Start a project in current directory")
                .long_about("\nStarts a project in current directory. \
                 If the directory is empty, creates a new project using npm (by default) and downloads catrina. \
                 If finds an existing project and catrina isn't install, installs catrina with npm (by default). \
                 If catrina is currently installed, just creates the config file.")
                .arg(
                    Arg::with_name("skip")
                        .short("d")
                        .long("default")
                        .help("Skip configuration questions"),
                )
                .arg(
                    Arg::with_name("yarn")
                        .long("yarn")
                        .help("Use yarn instead of npm."),
                ),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Build project if the config file is in the current directory")
                .long_about("Build project if the config file is in the current directory. \

                ")
                .arg(
                    Arg::with_name("minify")
                        .long("minify")
                        .help("Minify the result file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("minify")
                .about("Minify .js, .css and .html files")
                .arg(
                    Arg::with_name("input")
                        .help("Files to minify")
                        .short("i")
                        .long("input")
                        .value_name("INPUT-FILES")
                        .takes_value(true)
                        .multiple(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("join")
                .about("Concatenate files")
                .arg(
                    Arg::with_name("input")
                        .help("Files to join")
                        .short("i")
                        .long("input")
                        .value_name("INPUT-FILES")
                        .takes_value(true)
                        .multiple(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .help("Result file of concatenation")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT-FILE")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("bundle")
                .about("Experimental function for universal bundling .js and .css files")
                .arg(
                    Arg::with_name("input")
                        .help("File for bundling")
                        .short("i")
                        .long("input")
                        .value_name("INPUT-FILE")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .help("Bundle result file")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT-FILE")
                        .takes_value(true)
                        .required(true),
                ),
        );

    app.get_matches()
}

fn main() {
    color_eyre::install().expect("Error with color_eyre crate");
    let matches = app_matches();

    let mut subcommand = "";
    if let (sub, Some(_match)) = matches.subcommand() {
        subcommand = sub;
    }

    match subcommand {
        "build" => {
            println!("{} function runs...", subcommand);
        }
        "bundle" => {
            println!("{} function runs...", subcommand);
        }
        "init" => {
            println!("{} function runs...", subcommand);
        }
        "join" => {
            println!("{} function runs...", subcommand);
        }
        "minify" => {
            println!("{} function runs...", subcommand);
        }
        _ => {}
    }
}
