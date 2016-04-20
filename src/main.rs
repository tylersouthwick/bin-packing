extern crate clap;

use clap::{Arg, App, SubCommand};

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

    let sizeFile = matches.value_of("sizes").unwrap();

    println!("pack contents of {} with max size {}", sizeFile, max);
}
