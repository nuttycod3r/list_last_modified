#[macro_use]
extern crate clap;
extern crate walkdir;
extern crate chrono;

use std::process::exit;

use clap::{App, Arg};
use std::fs::File;
use std::io::{ Write};
use walkdir::WalkDir;
use chrono::{DateTime, Local};

const OPTION_INPUT_SHORT: &str = "d";
const OPTION_INPUT: &str = "dir";
const OPTION_OUTPUT_SHORT: &str = "o";
const OPTION_OUTPUT: &str = "output";

fn main() {
    let matches = App::new("list_last_modified")
        .version("1.0.0")
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name(OPTION_INPUT)
                .short(OPTION_INPUT_SHORT)
                .long(OPTION_INPUT)
                .value_name("directory")
                .multiple(false)
                .takes_value(true)
                .required(true)
                .help("Directory which is transversed to obtain the last modified timestamp."),
        )
        .arg(
            Arg::with_name(OPTION_OUTPUT)
                .short(OPTION_OUTPUT_SHORT)
                .long(OPTION_OUTPUT)
                .value_name("file")
                .multiple(false)
                .takes_value(true)
                .required(true)
                .help("Text file a list of files with theirs respective last modified timestamps."),
        )
        .get_matches();

    let input = matches.value_of(OPTION_INPUT).unwrap();
    let output = matches.value_of(OPTION_OUTPUT).unwrap();

    let mut vf = vec![];

    println!("Finding all files in {:?}", input);
    for res_entry in WalkDir::new(input).follow_links(false) {
        match res_entry {
            Ok(entry) => {
                let res_meta = entry.metadata();
                if res_meta.is_err() {
                    eprintln!(
                        "unable to get meta data from {:?} [{}]",
                        entry.path(),
                        res_meta.err().unwrap()
                    );
                    exit(1);
                }
                let meta = res_meta.unwrap();
                let res_last_mod = meta.modified();
                if res_last_mod.is_err() {
                    eprintln!(
                        "unable to get last modified timestamp from {:?} [{}]",
                        entry.path(),
                        res_last_mod.err().unwrap()
                    );
                    exit(1);
                }
                let last_mod = DateTime::<Local>::from(res_last_mod.unwrap());
                let spath = entry.path().to_str().unwrap().to_string();
                let info = PathInfo {
                    path: spath,
                    last_modified: last_mod,
                };
                vf.push(info);
            }
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }
    println!("Sorting the file paths based on the last modified timestamp");
    vf.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));

    println!("Writing filelist to {}", output);
    let mut file = match File::create(output) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("unable to open output file: {}", e);
            exit(1);
        }
    };

    for info in vf {
        let res_write = file.write_fmt(format_args!("{}\n", info));
        match res_write {
            Err(e) => {
                eprintln!("unable to write to file: {} [error: {}]", info, e);
            },
            _ => {},
        }
    }
}

struct PathInfo {
    path: String,
    last_modified: DateTime<Local>,
}

impl std::fmt::Display for PathInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("{:?} {:?}", self.last_modified, self.path).as_str())
    }
}
