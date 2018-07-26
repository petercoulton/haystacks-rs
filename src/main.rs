extern crate clap;

use clap::App;
use clap::Arg;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let matches =
        App::new("rust-haystacks")
            .version("1.0")
            .author("Peter Coulton <petercoulton@gmail.com>")
            .arg(Arg::with_name("needles")
                .short("n")
                .long("needles")
                .value_name("FILE")
                .takes_value(true)
                .required(true))
            .get_matches();

    let needles_path = matches.value_of("needles").unwrap();

    let mut needles = HashSet::new();
    let needle_file = File::open(needles_path).expect("Unable to open needles file");
    for line in BufReader::new(needle_file).lines() {
        let line = line.expect("Unable to read needles line");
        needles.insert(line);
    }

    for line in BufReader::new(io::stdin()).lines() {
        let line = line.expect("Unable to read haystack line");
        if needles.contains(&line) {
            println!("{}", line);
        }
    }

}
