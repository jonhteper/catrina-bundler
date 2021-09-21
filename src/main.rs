use crate::args::CatrinaArgs;
use crate::catrina::{catrina_tool, VERSION_APP};
use clap::{App, Arg};

mod args;
mod catrina;

#[macro_use]
extern crate serde_derive;
extern crate clap;

fn main() {
    let matches = App::new("Catrina")
        .version(VERSION_APP)
        .author("jonhteper <jonhteper@triamseletea.com>")
        .about("A mini bundler for npm catrina package")
        .arg(
            Arg::with_name("ACTION")
                .help("The principal order by Catrina, you can use \"init\", \"build\", \"minify\" or \"bundle\"")
                .required(true)
                .index(1),
        )
        .arg(Arg::with_name("PATH").help("Filepath for \"minify\" command or \"combine\" command").index(2))
        .arg(Arg::with_name("PATH2").help("Filepath for second file in \"combine\" command").index(3))
        .arg(Arg::with_name("NAME").help("Filepath final file in \"combine\" command").index(4))
        .arg(
            Arg::with_name("skip")
                .short("s")
                .help("Skip configuration questions"),
        )
        .arg(
            Arg::with_name("yarn")
                .short("Y")
                .help("Use yarn instead of npm"),
        )
        .arg(
            Arg::with_name("minify")
                .short("m")
                .help("Minify the result file for \"bundle\" command"),
        )
        .get_matches();

    // new, update, build, run or version
    let args = CatrinaArgs {
        action: matches.value_of("ACTION").expect("Action its necessary"),
        filepath_1: matches.value_of("PATH").unwrap_or(""),
        filepath_2: matches.value_of("PATH2").unwrap_or(""),
        filename: matches.value_of("NAME").unwrap_or(""),
        skip: matches.is_present("skip"),
        yarn: matches.is_present("yarn"),
        minify: matches.is_present("minify"),
    };

    match catrina_tool(args) {
        Err(e) => panic!("{:?}", e),
        _ => println!("No errors"),
    }
}
