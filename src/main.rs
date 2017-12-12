#![feature(plugin)]
#![feature(custom_attribute)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]
#![recursion_limit="128"]

// Base logging crate
#[macro_use] extern crate log;

// yaml/json (de)serialization
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;

// webserver
extern crate rocket;
extern crate rocket_contrib;

// Templates
extern crate tera;

// lazy instantiation
// #[macro_use] extern crate lazy_static;

//Common data structure
extern crate motorsport_calendar_common;

// Logging
extern crate log4rs;

// arg parsing
extern crate clap;

// sqlite3 ORM
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;

// utility to load env files
extern crate dotenv;

// Chrono for time and date 
extern crate chrono;

// For extra iteration functions
extern crate itertools;

mod webserver;
mod schema;
mod model;
mod database;
mod config;
mod admin;

use clap::{Arg, App, ArgMatches};
use config::Config;

// Main used to quickly test things ;)
// fn main() {
//     use schema::*;
//     use model::Event as MEvent;
//     use model::Session as MSession;
//     use diesel::prelude::*;
//     let connection = database::establish_connection();
//     // let my_events: Vec<(MEvent, Option<MSession>)> = events::table.left_join(sessions::table).load(&connection).expect("Error loading events");
//     let sport_types: Vec<String> = events::table.select(events::sport).group_by(events::sport).load(&connection).expect("Error loading events");
//     for s in sport_types {
//         println!("~> {}", s);
//     }
// }

fn main() {
    let matches = get_matches();
    let config = matches.value_of("config").unwrap_or("conf.yml");
    let log4rs_config = matches.value_of("logconfig").unwrap_or("log4rs.yml");

    if matches.is_present("admin mode") {
        admin::launch_admin_pages();
    } else {

        log4rs::init_file(&log4rs_config, Default::default()).unwrap();
        match run(config) {
            Ok(()) => std::process::exit(0),
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            },
        };
    }
}

fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("Motorsport calendar API")
        .version("1.0")
        .author("Michael A. <mickjohnashe@hotmail.com>")
        .about("A restful api that serves the time and date of motorsport events")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("logconfig")
             .short("l")
             .long("logconfig")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("admin mode")
             .short("a")
             .long("admin_mode")
             .help("Launch in admin mode (use lcoalhost to CRUD events in the DB"))
        .get_matches()
}

fn run (conf_file: &str) -> Result<(),String> {
    info!("Starting up!");
    info!("About to launch rocket webserver");
    let config = load_config(conf_file)?;
    webserver::start();
    Ok(())
}

fn load_config(conf_file: &str) -> Result<Config, String> {
    Config::init_config_from_file(conf_file).map_err(|e| e.to_string())
}
