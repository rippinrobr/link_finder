#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate clap; 

use regex::Regex;
use std::process;
use std::env;
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
        
        //println!("Checking file {} for links", input_path);
        //let re = Regex::new(r"(\[.*\].http.*\)).*").unwrap();
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
                            //if !&links.contains(&&l[bindex..]) {
                                println!("{})", &l[bindex..]);
                                // let r = &l[bindex..];
                                // links.push(r.to_owned());
                            //}
                        }
                    }
                }
            }
        }
        // 
        // println!("number of matches: {}", captured.len());
        // println!("captured[0]: {}", captured.get(0).map_or("", |m| m.as_str()));
        // println!("captured[1]: {}", captured.get(1).map_or("", |m| m.as_str()));
        // for cap in re.captures_iter(&contents).next() {
        //     for c in cap.iter().next() {
        //         match c {
        //             Some(x) => println!("x: {}", x.as_str()),
        //             None => println!("None")
        //         }
        //     }
        // }
    }

    //println!("Hello, world!");
}
