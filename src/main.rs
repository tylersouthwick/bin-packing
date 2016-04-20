extern crate clap;

use clap::{Arg, App};

use std::iter::FromIterator;
use std::io::Error;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    let matches = App::new("bin-packer")
                          .version("1.0")
                          .author("Tyler Southwick <tyler@northfuse.net>")
                          .about("packs units in a bin")
                          .arg(Arg::with_name("sizes")
                               .required(true)
                               .short("s")
                               .long("sizes")
                               .value_name("FILE")
                               .help("loads measurements from file")
                               .takes_value(true))
                          .arg(Arg::with_name("binSize")
                               .help("bin size")
                               .required(true)
                               .index(1))
                          .get_matches();

    let max = matches.value_of("binSize").unwrap();

    let size_file = matches.value_of("sizes").unwrap();

    println!("pack contents of {} with max size {}", size_file, max);

    let lines = process_file(size_file).unwrap();

    println!("found {} lines", lines.len());
    for x in lines {
        println!("line: {}", x);
    }
}

fn process_file(file_name : &str) -> Result<Vec<u32>, Error> {
    let mut f = try!(File::open(file_name));
    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    let rawList = contents.split("\n")
            .map(|s : &str| match s.parse::<u32>() {
                Ok(n) => n,
                Err(e) => 0
            })
            .collect::<Vec<u32>>()
            .iter()
            .filter(|&x| *x > 0)
            .map(|&x| x)
            .collect::<Vec<u32>>();

    Ok(rawList)
}

