extern crate motorsport_calendar_common;

use motorsport_calendar_common::event::*;
use serde_yaml;
use std::io::prelude::*;
use std::fs::File;

pub fn load_events_from_yml_files(yaml_files: &[String]) -> Result<Vec<Event>, String> {
    let mut events = Vec::new();
    for yaml_file in yaml_files {
        let mut new_events = try!(load_events_from_yml_file(yaml_file));
        events.append(&mut new_events);
    }
    Ok(events)
}

pub fn load_events_from_yml_file(yaml_file: &str) -> Result<Vec<Event>, String> {
    info!("Getting events from yaml file {}", yaml_file);
    let mut f = try!(File::open(yaml_file)
                     .map_err(|e| format!("{}:{}", yaml_file, e.to_string())));
    let mut s = String::new();
    try!(f.read_to_string(&mut s)
         .map_err(|e| format!("{}:{}", yaml_file, e.to_string())));
    let rounds = try!(serde_yaml::from_str(&s)
                      .map_err(|e| format!("{}:{}", yaml_file, e.to_string())));
    Ok(rounds)
}

