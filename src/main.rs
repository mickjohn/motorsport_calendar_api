#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate rocket;
#[macro_use] extern crate lazy_static;
extern crate motorsport_calendar_common;

mod config;
mod data;
mod webserver;
mod event_loader;

use motorsport_calendar_common::event::*;
use config::Config;
use data::json_data;
use std::{thread, time};

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

    json_data::init(&events);

    if config.enable_data_refresh() {
        info!("polling of data files enabled");
        thread::spawn(move || {
            poll_yml_files(&config);
        });
    }

    info!("About to launch rocket webserver");
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
