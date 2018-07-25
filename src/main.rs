extern crate regex;
extern crate clap; 

use std::process;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let _matches = clap::App::new("link_finder")
       .version("0.0.1")
       .about("Parses Markdown files looking for Links and creates a list of all links found in the file")
       .author("Rob Rowe")
       .arg(clap::Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("MARKDOWN FILE")
        .help("A path to a markdown file")
        .takes_value(true)
        .required(true))
    .get_matches();

    if let Some(input_path) = _matches.value_of("input")  {
        if input_path == "" {
            println!("parse a non-existant file!");
            process::exit(1);
        }
        
        let mut f = File::open(input_path).expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        for line in contents.lines() {
            let sp = line.split(")");
            if sp.clone().count() > 1 {
                for l in sp {
                    if l != "" {
                        if let Some(bindex) = l.find("[") {
                            println!("{})", &l[bindex..]);
                        }
                    }
                }
            }
        }
    }
}
