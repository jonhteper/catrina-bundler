use crate::catrina::{catrina_tool, VERSION_APP};
use clap::{App, Arg, SubCommand};
use std::env;

mod catrina;
#[macro_use]
extern crate serde_derive;
extern crate clap;

fn main() {
    let matches = App::new("Catrina")
        .version(VERSION_APP)
        .arg(
            Arg::with_name("ACTION")
                .help("The principal order by Catrina")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("NAME")
                .help("Indicates the name of project or version of std library")
                .index(2),
        )
        .arg(
            Arg::with_name("skip")
                .short("s")
                .help("Skip configuration questions"),
        )
        .get_matches();

    // new, update, build, run or version
    let action = matches.value_of("ACTION").expect("Action its necessary");
    let name = matches.value_of("NAME").unwrap_or("");
    let skip = matches.is_present("skip");
    let args = (action, name, skip);

    match catrina_tool(args) {
        Err(e) => panic!("{:?}", e),
        _ => println!("No errors"),
    }
}
