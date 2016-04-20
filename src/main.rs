extern crate clap;

use clap::{Arg, App};

use std::io::Error;
use std::io::prelude::*;
use std::fs::File;

fn main() {
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
                          .arg(Arg::with_name("tolerance")
                               .help("bin size")
                               .short("t")
                               .takes_value(true)
                               .required(true))
                          .get_matches();

    let max = matches.value_of("binSize").unwrap().parse::<u32>().unwrap();

    let size_file = matches.value_of("sizes").unwrap();

    let tolerance = matches.value_of("tolerance").unwrap().parse::<u32>().unwrap();

    println!("pack contents of {} with max_size={} and tolerance={}", size_file, max, tolerance);

    let lines = process_file(size_file).unwrap();

    println!("found {} lines", lines.len());

    let bins = bin_pack(max, tolerance, lines);

    println!("bins={}", bins);
}

fn process_file(file_name : &str) -> Result<Vec<u32>, Error> {
    let mut f = try!(File::open(file_name));
    let mut contents = String::new();
    try!(f.read_to_string(&mut contents));
    let raw_list = contents.split("\n")
            .map(|s : &str| match s.parse::<u32>() {
                Ok(n) => n,
                Err(_) => 0
            })
            .collect::<Vec<u32>>()
            .iter()
            .filter(|&x| *x > 0)
            .map(|&x| x)
            .collect::<Vec<u32>>();

    Ok(raw_list)
}

fn bin_pack(max : u32, tolerance : u32, sizes : Vec<u32>) -> u32 {
    println!("bin packing {} elements into size {} with tolerance={}", sizes.len(), max, tolerance);

    let mut new_sizes : Vec<u32>= Vec::new();
    for size in sizes {
        split_num(max, size, &mut new_sizes);
    }
    new_sizes.sort_by(|x, y| y.cmp(x));

    let bins = new_sizes
        .iter()
        //.fold((0, 0), |result, item| (result.0 + 1, *item));
        //.fold((0, 0), |result, item| (result.0, result.1 + *item));
        .fold((0, 0), |result, item| {
              if result.1 + item > max {
                  (result.0 + 1, *item)
              } else {
                  (result.0, result.1 + *item + tolerance)
              }
        });
        //.1;

    println!("bins.0={} bins.1={}", bins.0, bins.1);

    if bins.1 > 0 {
        bins.0 + 1
    } else {
        bins.0
    }
}

fn split_num(max : u32, num : u32, vec : &mut Vec<u32>) {
    let mut x = num;
    //println!("\nstarting num={}", num);
    while x > 0 {
        let y = min(max, x);
        //println!("num={} x={} y={}", num, x, y);
        x = x - y;
        vec.push(y);
    }
    //println!("ending num={}\n", num);
}

fn min(a : u32, b : u32) -> u32 {
    if a < b {
        a
    } else {
        b
    }
}
