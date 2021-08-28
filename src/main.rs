use crate::args::CatrinaArgs;
use crate::catrina::{catrina_tool, VERSION_APP};
use clap::{App, Arg, SubCommand};
use std::env;

mod args;
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
        .arg(Arg::with_name("PARAM").help("Optional param").index(2))
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
        .get_matches();

    // new, update, build, run or version
    let args = CatrinaArgs {
        action: matches.value_of("ACTION").expect("Action its necessary"),
        param: matches.value_of("PARAM").unwrap_or(""),
        skip: matches.is_present("skip"),
        yarn: matches.is_present("yarn"),
    };

    match catrina_tool(args) {
        Err(e) => panic!("{:?}", e),
        _ => println!("No errors"),
    }
}
