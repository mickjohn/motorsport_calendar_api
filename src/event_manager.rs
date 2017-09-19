#![feature(plugin)]
#[macro_use] extern crate log;
extern crate env_logger;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate serde_json;
#[macro_use] extern crate lazy_static;
extern crate motorsport_calendar_common;
extern crate clap;

mod config;
mod event_loader;

use motorsport_calendar_common::event::*;
use config::Config;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Event Manager")
        .version("1.0")
        .author("Michael A. <mickjohnashe@hotmail.com>")
        .about("A cli tool for looking at the event yaml files")
        .arg(Arg::with_name("sport_type")
             .short("s")
             .long("sport-type")
             .value_name("SPORT TPYE")
             .help("limit results to this sport type, e.g -s DTM, or -s 'Formula 1'")
             .takes_value(true))
        .get_matches();

    let sport_type = matches.value_of("sport_type");
    
    env_logger::init().unwrap();
    match run(sport_type) {
        Ok(()) => {
            std::process::exit(0);
        },
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        },
    };
}

fn run(sport_type: Option<&str>) -> Result<(), String> {
    let conf_file = "conf.yml";
    let (mut events, _) = try!(get_events_and_config(conf_file));

    if let Some(sport) = sport_type {
        events = events.into_iter().filter(|e| e.sport == sport).collect::<Vec<Event>>();
    }

    pretty_print_events(&events);
    Ok(())
}

fn get_events_and_config(conf_file: &str) -> Result<(Vec<Event>, Config), String> {
    let config = try!(Config::init_config_from_file(conf_file)
                      .map_err(|e| format!("Could not load config file{}, reason: {}", conf_file, e)));

    let events = try!(event_loader::load_events_from_yml_files(&config.data_paths()));
    Ok((events,config))
}

fn pretty_print_events(events: &[Event]) {
    let event_types = {
        let mut event_types = events.iter().map(|e| e.sport.clone()).collect::<Vec<String>>();
        event_types.sort();
        event_types.dedup();
        event_types
    };

    for et in &event_types {
        println!("{}", et);
        let events_of_type = get_events_of_type(events, et);
        for line in events_of_type.render() {
            println!("{}", line);
        }
        // for (i,e) in events_of_type.iter().enumerate() {
        //     let is_last = (i+1) == events_of_type.len();
        //     let (event_prefix, session_prefix) = match is_last {
        //         true => ("┗"," "),
        //         false => ("┣", "┃"),
        //     };

        //     let line = format!("{}:{}:{}", e.round, e.country, e.location);
        //     println!("{} {}", event_prefix, line);

        //     let session_lines = format_sessions(e);
        //     for s_line in session_lines {
        //         println!("{} {}", session_prefix, s_line);
        //     }
        // }
        println!("");
    }
}

fn format_sessions(e: &Event) -> Vec<String> {
    let mut lines = Vec::new();
    for (i,s) in e.sessions.iter().enumerate() {
        let is_last = (i+1) == e.sessions.len();
        let line = if is_last {
            format!("┗ {}", s.name)
        } else {
            format!("┣ {}", s.name)
        };
        lines.push(line);
    }
    lines
}

fn get_events_of_type<'a>(events: &'a[Event], t: &str) -> Vec<&'a Event> {
    events.iter()
        .filter(|e| e.sport == t)
        .collect::<Vec<&Event>>()
}

trait TreePrint {
    fn render(&self) -> Vec<String>;
}

impl TreePrint for Vec<Session> {
    fn render(&self) -> Vec<String> {
        let mut lines = Vec::new();
        for (i,s) in self.iter().enumerate() {
            let is_last = (i+1) == self.len();
            let line = match is_last {
                true => format!("┗ {}", s.name),
                false => format!("┣ {}", s.name),
            };
            lines.push(line);
        }
        lines
    }
}

impl <'a>TreePrint for Vec<&'a Event> {
    fn render(&self) -> Vec<String> {
        let mut lines = Vec::new();
        for (i,e) in self.iter().enumerate() {
            let is_last = (i+1) == self.len();
            let (event_prefix, session_prefix) = match is_last {
                true => ("┗"," "),
                false => ("┣", "┃"),
            };

            let line = format!("{}:{}:{}", e.round, e.country, e.location);
            lines.push(line);
            for session_line in e.sessions.render() {
                lines.push(format!("{} {}", session_prefix, session_line));
            }
        }
        lines
    }
}

impl TreePrint for Vec<Event> {
    fn render(&self) -> Vec<String> {
        let mut lines = Vec::new();
        for (i,e) in self.iter().enumerate() {
            let is_last = (i+1) == self.len();
            let (event_prefix, session_prefix) = match is_last {
                true => ("┗"," "),
                false => ("┣", "┃"),
            };

            let line = format!("{}:{}:{}", e.round, e.country, e.location);
            lines.push(line);
            for session_line in e.sessions.render() {
                lines.push(format!("{} {}", session_prefix, session_line));
            }
        }
        lines
    }
}
