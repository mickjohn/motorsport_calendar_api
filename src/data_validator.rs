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
mod event_loader;

use motorsport_calendar_common::event::*;
use config::Config;

fn main() {
    println!("data validator!");
    match run() {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        },
    };
}

fn run() -> Result<(), String> {
    let conf_file = "conf.yml";
    let (events, _) = try!(get_events_and_config(conf_file));

    let event_types = {
        let mut event_types = events.iter().map(|e| e.sport.as_str()).collect::<Vec<&str>>();
        event_types.sort();
        event_types.dedup();
        event_types
    };

    let mut errors = Vec::new();
    for event_type in event_types {
        let events_for_sport = get_events_of_type(&events, event_type);

        match verify_round_numbers(&events_for_sport) {
            Err(e) => errors.push(e),
            _ => (),
        };
    }

    Ok(())
}

fn get_events_of_type(events: &[Event], t: &str) -> Vec<Event> {
    events.iter().filter(|&e| e.sport == t).map(|e| e.clone()).collect::<Vec<Event>>()
}

fn verify_round_numbers(events: &[Event]) -> Result<(), String> {
    let mut last_num: Option<u64> = None;
    for e in events {
        if let Some(num) = last_num {
            if (num + 1) != e.round {
                let e = format!("round numbers are not sequential. Expected {} to appear after {}, for event {:?}", num + 1, num, e);
                return Err(e)
            }
        } else {
            last_num = Some(e.round)
        }
    }
    Ok(())
}

// fn verify_sequential_dates(events: &[Event]) -> Result<(), String> {
// }

fn get_events_and_config(conf_file: &str) -> Result<(Vec<Event>, Config), String> {
    let config = try!(Config::init_config_from_file(conf_file)
                      .map_err(|e| format!("Could not load config file{}, reason: {}", conf_file, e)));

    let events = try!(event_loader::load_events_from_yml_files(&config.data_paths()));
    Ok((events,config))
}

//Rules 
// 1) No duplicate events or session
// 2) No duplicate round numbers per sport
// 3) Round numbers and dates are sequential. e.g Round 3 happens before round 4
