#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
extern crate csv;
extern crate rustc_serialize;
#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

mod config;
mod event;
mod data;
mod webserver;
mod round;

use event::Event;
use config::Config;
use data::json_data;
use std::{thread, time};
use round::*;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    env_logger::init().unwrap();
    test_rounds();
    // convert();
    // match run() {
    //     Ok(()) => std::process::exit(0),
    //     Err(e) => {
    //         println!("{}", e);
    //         std::process::exit(1);
    //     },
    // };
}

fn test_rounds() {
    let mut f = File::open("res.yaml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let rounds: Vec<Round> = serde_yaml::from_str(&s).unwrap();
    println!("{:?}", rounds);
}

fn convert() {
    let conf_file = "conf.yml";
    let (events, _) = get_events_and_config(conf_file).unwrap();

    let mut r = 1;
    let mut t: Vec<Round> = Vec::new();
    let mut es: Vec<Event> = Vec::new();
    for e in events {
        if r != e.round {
            let mut sessions = Vec::new();
            for e in &es {
                sessions.push( Session {
                    name: e.event_name.clone(),
                    date: e.date.clone(),
                    time: e.start_time.clone(),
                });
            }

            let temp_event = es[0].clone();
            let round = Round {
                sport: temp_event.sport.clone(),
                round: temp_event.round.clone(),
                country: temp_event.country.clone(),
                location: temp_event.track_name.clone(),
                sessions: sessions,
            };

            t.push(round);
            r = e.round.clone();
            es = Vec::new();
        } else {
            es.push(e.clone());
        }
    }

    let y = serde_yaml::to_string(&t).unwrap();
    println!("{}", y);

}

fn run () -> Result<(),String> {
    info!("Starting up!");
    let conf_file = "conf.yml";
    let (events, config) = try!(get_events_and_config(conf_file));

    info!("About to initialise hashmaps");
    json_data::init(&events);
    info!("Data initalisation complete");

    // thread::spawn(move || {
    //     poll_csv_files(&config);
    // });

    info!("About to launch rocket webserver");
    webserver::start();
    Ok(())
}

fn get_events_and_config(conf_file: &str) -> Result<(Vec<Event>, Config), String> {
    let config = try!(Config::init_config_from_file(conf_file)
                      .map_err(|e| format!("Could not load config file{}, reason: {}", conf_file, e)));
    info!("Loaded config file {}", conf_file);

    info!("About to load events from these files: {:?}", config.data_paths);
    let events = try!(load_events_from_csv_files(&config.data_paths)
                      .map_err(|e| format!("Error while loading events, reason: {}", e)));

    info!("Finished loading events from csv files, got {} events", events.len());
    Ok((events, config))
}

fn load_events_from_csv_files(csv_files: &[String]) -> Result<Vec<Event>, String> {
    let mut events = Vec::new();
    for csv_file in csv_files {
        let mut new_events = try!(load_events_from_csv_file(csv_file));
        events.append(&mut new_events);
    }
    Ok(events)
}

fn load_rounds_from_yml_files(yaml_files: &[String]) -> Result<Vec<Round>, String> {
    let mut events = Vec::new();
    for yaml_file in yaml_files {
        let mut new_events = try!(load_events_from_yml_file(yaml_file));
        events.append(&mut new_events);
    }
    Ok(events)
}

fn load_events_from_csv_file(csv_file: &str) -> Result<Vec<Event>, String> {
    info!("Getting events from csv file {}", csv_file);
    let mut rdr = try!(csv::Reader::from_file(csv_file).map_err(|e| e.to_string()));
    let mut events = Vec::new();
    for event in rdr.decode() {
        let e : Event = try!(event.map_err(|e| e.to_string()));
        debug!("got event: {:?}", e);
        events.push(e);
    }
    Ok(events)
}

fn load_events_from_yml_file(yaml_file: &str) -> Result<Vec<Round>, String> {
    info!("Getting events from yaml file {}", yaml_file);
    let mut f = try!(File::open(yaml_file).map_err(|e| e.to_string()));
    let mut s = String::new();
    try!(f.read_to_string(&mut s).map_err(|e| e.to_string()));
    let rounds = try!(serde_yaml::from_str(&s).map_err(|e| e.to_string()));
    Ok(rounds)
}

fn poll_csv_files(config: &Config) {
    let time = time::Duration::from_millis(1000*5);
    loop {
        thread::sleep(time);
        let events = load_events_from_csv_files(&config.data_paths).unwrap();
        info!("Refreshing events now...");
        json_data::init(&events);
        info!("Finished refreshing!");
    }
}
