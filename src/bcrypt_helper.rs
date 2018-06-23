// arg parsing
extern crate clap;
// Bcrypt for password hashing
extern crate bcrypt;

use bcrypt::{hash, DEFAULT_COST};
use clap::{App, Arg};
use std::io::{self, Read};

fn main() {
    let matches = App::new("Motorsport calendar API - bcrypt helper")
        .version("1.0")
        .author("Michael A. <mickjohnashe@hotmail.com>")
        .about("A CLI tool to generate and verify bcrypt hashes")
        .arg(
            Arg::with_name("generate")
                .short("g")
                .long("gen")
                .help("Read from stdin and generate bcrypt hash")
                .required(true),
        )
        .get_matches();

    if matches.is_present("generate") {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        print!("{}", hash("hunter2", DEFAULT_COST).unwrap());
    }
}
