#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
extern crate chrono;

mod config;
mod event;
mod data;
mod webserver;

use event::Event;
use config::Config;
use data::json_data;
use std::{thread, time};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    env_logger::init().unwrap();
    match run() {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        },
    };
}

fn run () -> Result<(),String> {
    info!("Starting up!");
    let conf_file = "conf.yml";
    let (events, config) = try!(get_events_and_config(conf_file));

    info!("About to initialise hashmaps");
    json_data::init(&events);
    info!("Data initalisation complete");

    thread::spawn(move || {
        poll_yml_files(&config);
    });

    info!("About to launch rocket webserver");
    webserver::start();
    Ok(())
}

fn get_events_and_config(conf_file: &str) -> Result<(Vec<Event>, Config), String> {
    let config = try!(Config::init_config_from_file(conf_file)
                      .map_err(|e| format!("Could not load config file{}, reason: {}", conf_file, e)));
    info!("Loaded config file {}", conf_file);

    info!("About to load events from these files: {:?}", config.data_paths);
    let events = try!(load_events_from_yml_files(&config.data_paths)
                      .map_err(|e| format!("Error while loading events, reason: {}", e)));

    info!("Finished loading events from yml files, got {} events", events.len());
    Ok((events, config))
}

fn load_events_from_yml_files(yaml_files: &[String]) -> Result<Vec<Event>, String> {
    let mut events = Vec::new();
    for yaml_file in yaml_files {
        let mut new_events = try!(load_events_from_yml_file(yaml_file));
        events.append(&mut new_events);
    }
    Ok(events)
}

fn load_events_from_yml_file(yaml_file: &str) -> Result<Vec<Event>, String> {
    info!("Getting events from yaml file {}", yaml_file);
    let mut f = try!(File::open(yaml_file).map_err(|e| e.to_string()));
    let mut s = String::new();
    try!(f.read_to_string(&mut s).map_err(|e| e.to_string()));
    let rounds = try!(serde_yaml::from_str(&s).map_err(|e| e.to_string()));
    Ok(rounds)
}

fn poll_yml_files(config: &Config) {
    let time = time::Duration::from_millis(1000*60*5);
    loop {
        thread::sleep(time);
        let events = load_events_from_yml_files(&config.data_paths).unwrap();
        info!("Refreshing events now...");
        json_data::init(&events);
        info!("Finished refreshing!");
    }
}
