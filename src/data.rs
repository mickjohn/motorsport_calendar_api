use std::collections::HashMap;
use std::sync::RwLock;
use motorsport_calendar_common::event::*;

// Types for holding pre-serialized data.
type EventsType = HashMap<String, String>;
type EventTypeRound = HashMap<(String, u64), String>;
type EventTypeRoundNum = HashMap<(String, u64, u64), String>;

#[derive(Debug)]
pub struct Data {
    pub events: String,
    pub events_type: EventsType,
    pub events_type_round: EventTypeRound,
    pub events_type_round_num: EventTypeRoundNum,
}

impl Data {
    pub fn new() -> Data {
        Data {
            events: String::new(),
            events_type: HashMap::new(),
            events_type_round: HashMap::new(),
            events_type_round_num: HashMap::new(),
        }
    }
}

/*
 * The purpose of this module is to pre-serialize all of the data before it's needed.
 * The serialized data is then put into a number of hashmaps for easy querying.
 * This is done to stop the vector of events from being searched and serialzed every time 
 * a request comes in.
 */

fn create_type_map(events: &[Event]) -> HashMap<&str, Vec<&Event>> {
    let mut map: HashMap<&str, Vec<&Event>> = HashMap::new();
    for e in events {
        let key: &str = &e.sport;
        let mut v = map.entry(key).or_insert_with(Vec::new);
        v.push(e);
    }
    map
}

fn create_type_and_round_map(events: &[Event]) -> HashMap<(&str, &u64), &Event> {
    let mut map: HashMap<(&str, &u64), &Event> = HashMap::new();
    for e in events {
        let key: (&str, &u64) = (&e.sport, &e.round);
        map.insert(key, e);
    }
    map
}

fn create_type_round_and_number_map(events: &[Event]) -> HashMap<(&str, &u64, u64), &Session> {
    let mut map: HashMap<(&str, &u64, u64), &Session> = HashMap::new();
    for e in events {
        for(i,s) in e.sessions.iter().enumerate() {
            let key: (&str, &u64, u64) = (&e.sport, &e.round, (i + 1) as u64);
            map.insert(key, s);
        }
    }
    map
}

pub mod json_data {
    use super::*;
    use serde_json;

    lazy_static! {
        pub static ref DATA: RwLock<Data> = RwLock::new(Data::new());
    }

    pub fn init(events: &[Event]) {
        info!("Beginning json data initialisation");
        let data = Data {
            events: create_events_json(events),
            events_type: create_events_type_json_map(events),
            events_type_round: create_events_type_round_json_map(events),
            events_type_round_num: create_events_type_round_num_json_map(events),
        };
        let mut d = DATA.write().unwrap();
        *d = data;
        info!("json data initialisation complete");
    }

    fn create_events_json(events: &[Event]) -> String {
        info!("Initialising events json string");
        serde_json::to_string(&events).unwrap()
    }

    fn create_events_type_json_map(events: &[Event]) -> EventsType {
        info!("Initialising (event type) -> (json) map");
        let map = create_type_map(events);
        let mut json_map = EventsType::new();
        // Now serialize and insert the events into a (event type) -> (json events) map.
        for (sport_type, events) in &map {
            let json_events = serde_json::to_string(&events).unwrap();
            debug!("key = {}, value = {}", sport_type, json_events);
            json_map.insert(sport_type.to_string(), json_events);
        }
        json_map
    }

    fn create_events_type_round_json_map(events: &[Event]) -> EventTypeRound {
        info!("Initialising (event type, round) -> (json) map");
        let map = create_type_and_round_map(events);
        let mut json_map = EventTypeRound::new();
        // Now serialize and insert the events into a (event type, round) -> (json events) map.
        for ((sport_type, round), events) in map {
            let json_events = serde_json::to_string(&events).unwrap();
            let key = (sport_type.to_string(), *round);
            debug!("key = {:?}, value = {}", key, json_events);
            json_map.insert(key, json_events);
        }
        json_map
    }

    fn create_events_type_round_num_json_map(events: &[Event]) -> EventTypeRoundNum {
        info!("Initialising (event type, round, num) -> (json) map");
        let map = create_type_round_and_number_map(events);
        let mut json_map = EventTypeRoundNum::new();
        // Now serialize and insert the events into a (event type, round, num) -> (json events) map.
        for ((sport_type, round, num), events) in map {
            let json_events = serde_json::to_string(&events).unwrap();
            let key = (sport_type.to_string(), *round, num);
            debug!("key = {:?}, value = {}", key, json_events);
            json_map.insert(key, json_events);
        }
        json_map
    }
}
