#![feature(plugin)]
#![feature(custom_attribute)]
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

// lazy instantiation
#[macro_use] extern crate lazy_static;

//Common data structure
extern crate motorsport_calendar_common;

// Logging
#[macro_use] extern crate log4rs;

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
#[macro_use] extern crate itertools;

mod config;
mod data;
mod webserver;
mod event_loader;
mod schema;
mod model;
mod database;

use motorsport_calendar_common::event::*;
use config::Config;
use data::json_data;
use std::{thread, time};
use clap::{Arg, App};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use model::{Event as MEvent, Session as MSession};
use itertools::Itertools;

// fn main() {
//     use schema::events::dsl::*;
//     use schema::sessions::dsl::*;
//     use schema::datetest::dsl::*;
//     let connection = database::establish_connection();
//     let my_events: Vec<(MEvent, Option<MSession>)> = events.left_join(sessions).load(&connection).expect("Error loading events");

//     for (key,group) in my_events.iter().group_by(|t| t.0.id).into_iter() {
//         let mut cevents = Vec::new();
//         for &(ref ev, ref session) in group {
//             let mut msessions = Vec::new();
//             if session.is_some() {
//                 let s = session.as_ref().unwrap().clone();
//                 msessions.push(s);
//             }
//             let e = model::from_model(ev.clone(), msessions);
//             cevents.push(e);
//         }
//         println!("EVENT = {:?}", cevents);
//     }
// }

fn main() {
    let matches = App::new("Motorsport calendar API")
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
        .get_matches();
    let config = matches.value_of("config").unwrap_or("conf.yml");
    let log4rs_config = matches.value_of("logconfig").unwrap_or("log4rs.yml");

    log4rs::init_file(&log4rs_config, Default::default()).unwrap();
    match run(&config) {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        },
    };
}

fn run (conf_file: &str) -> Result<(),String> {
    info!("Starting up!");
    // let (events, config) = try!(get_events_and_config(conf_file));

    // json_data::init(&events);

    // if config.enable_data_refresh() {
    //     info!("polling of data files enabled");
    //     thread::Builder::new().name("event_polling".to_string()).spawn(move || {
    //         poll_yml_files(&config);
    //     });
    // }

    // info!("About to launch rocket webserver");
    webserver::start();
    Ok(())
}

fn get_events_and_config(conf_file: &str) -> Result<(Vec<Event>, Config), String> {
    let config = try!(Config::init_config_from_file(conf_file)
                      .map_err(|e| format!("Could not load config file{}, reason: {}", conf_file, e)));
    info!("Loaded config file {}", conf_file);

    info!("About to load events from these files: {:?}", config.data_paths());
    let events = try!(event_loader::load_events_from_yml_files(&config.data_paths())
                      .map_err(|e| format!("Error while loading events, reason: {}", e)));

    info!("Finished loading events from yml files, got {} events", events.len());
    Ok((events, config))
}

fn poll_yml_files(config: &Config) {
    let seconds = config.data_refresh_interval_seconds();
    let time = time::Duration::from_secs(seconds);
    info!("poll time for data files is {} seconds", seconds);
    loop {
        thread::sleep(time);
        match event_loader::load_events_from_yml_files(config.data_paths()) {
            Ok(events) => {
                info!("Refreshing events now...");
                json_data::init(&events);
                info!("Finished refreshing!");
            },
                Err(e) => error!("An error occured while refreshing the events from the config files, reason: {}", e),
        };
    }
}
