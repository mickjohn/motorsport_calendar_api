// arg parsing
extern crate clap;
// Bcrypt for password hashing
extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};
use clap::{App, Arg};
use std::io::{self, Read};

fn main() {
    let matches = App::new("Motorsport calendar API - bcrypt helper")
	.version("1.0")
	.author("Michael A. <mickjohnashe@hotmail.com>")
	.about("A CLI tool to generate and verify bcrypt hashes")
	.arg(Arg::with_name("generate")
	     .short("g")
	     .long("gen")
	     .help("Read from stdin and generate bcrypt hash")
	     .conflicts_with("check"))
	.arg(Arg::with_name("check")
	     .short("c")
	     .long("check")
	     .help("Check plaintext against hash")
	     .conflicts_with("gen"))
	.get_matches();

    if matches.is_present("generate") {
	generate_and_print_hash();
    } else if matches.is_present("check") {
	let plain = "qwerty";
	let hash = "hash goes here";
	if verify(&plain, &hash).unwrap() {
	    println!("plaintext verified against hash");
	} else {
	    println!("plaintext NOT verified against hash");
	}
    }
}

fn generate_and_print_hash() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    print!("{}", hash(&buffer, DEFAULT_COST).unwrap());
}
