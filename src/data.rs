use std::collections::HashMap;
use event::Event;

// Types for holding pre-serialized data.
type EventsType = HashMap<String, String>;
type EventTypeRound = HashMap<(String, u64), String>;
type EventTypeRoundNum = HashMap<(String, u64, u64), String>;

/*
 * The purpose of this module is to pre-serialize all of the data before it's needed.
 * The serialized data is then put into a number of hashmaps for easy querying.
 * This is done to stop the vector of events from being searched and serialzed every time 
 * a request comes in. The downside is that the events cannot be updated while the server is
 * running, and a restart is required to pick up changes.
 */

fn get_events_type_events_map(events: &[Event]) -> HashMap<&str, Vec<&Event>> {
    let mut map: HashMap<&str, Vec<&Event>> = HashMap::new();
    for e in events {
        let key: &str = &e.sport;
        let mut v = map.entry(key).or_insert_with(Vec::new);
        v.push(e);
    }
    map
}

fn get_events_round_events_map(events: &[Event]) -> HashMap<(&str, &u64), Vec<&Event>> {
    let mut map: HashMap<(&str, &u64), Vec<&Event>> = HashMap::new();
    for e in events {
        let key: (&str, &u64) = (&e.sport, &e.round);
        let mut v = map.entry(key).or_insert_with(Vec::new);
        v.push(e);
    }
    map
}

fn get_events_type_round_num_events_map(events: &[Event]) -> HashMap<(&str, &u64, &u64), Vec<&Event>> {
    let mut map: HashMap<(&str, &u64, &u64), Vec<&Event>> = HashMap::new();
    for e in events {
        let key: (&str, &u64, &u64) = (&e.sport, &e.round, &e.number_in_round);
        let mut v = map.entry(key).or_insert_with(Vec::new);
        v.push(e);
    }
    map
}

pub mod json_data {
    use super::*;
    use std::sync::Mutex;
    use std::collections::HashMap;
    use event::Event;
    use serde_json;

    lazy_static! {
        pub static ref EVENTS: Mutex<String> = Mutex::new(String::new());
        pub static ref EVENTS_TYPE: Mutex<EventsType> = Mutex::new(HashMap::new());
        pub static ref EVENTS_TYPE_ROUND: Mutex<EventTypeRound> = Mutex::new(HashMap::new());
        pub static ref EVENTS_TYPE_ROUND_NUM: Mutex<EventTypeRoundNum> = Mutex::new(HashMap::new());
    }

    pub fn init(events: &[Event]) {
        init_events(events);
        init_events_type(events);
        init_events_type_round(events);
        init_events_type_round_num(events);
    }

    fn init_events(events: &[Event]) {
        info!("Initialising events json string");
        let events_json = serde_json::to_string(&events).unwrap();
        let mut static_events = EVENTS.lock().unwrap();
        static_events.push_str(&events_json);
    }

    fn init_events_type(events: &[Event]) {
        info!("Initialising (event type) -> (json) map");
        let map = get_events_type_events_map(events);

        // Now serialize and insert the events into a (event type) -> (json events) map.
        let mut static_map = EVENTS_TYPE.lock().expect("Error aquiring lock");
        for (sport_type, events) in &map {
            let json_events = serde_json::to_string(&events).unwrap();
            debug!("key = {}, value = {}", sport_type, json_events);
            static_map.insert(sport_type.to_string(), json_events);
        }
    }

    fn init_events_type_round(events: &[Event]) {
        info!("Initialising (event type, round) -> (json) map");
        let map = get_events_round_events_map(events);

        // Now serialize and insert the events into a (event type, round) -> (json events) map.
        let mut static_map = EVENTS_TYPE_ROUND.lock().expect("Error aquiring lock");
        for ((sport_type, round), events) in map {
            let json_events = serde_json::to_string(&events).unwrap();
            let key = (sport_type.to_string(), *round);
            debug!("key = {:?}, value = {}", key, json_events);
            static_map.insert(key, json_events);
        }
    }

    fn init_events_type_round_num(events: &[Event]) {
        info!("Initialising (event type, round, num) -> (json) map");
        let map = get_events_type_round_num_events_map(events);

        // Now serialize and insert the events into a (event type, round, num) -> (json events) map.
        let mut static_map = EVENTS_TYPE_ROUND_NUM.lock().expect("Error aquiring lock");
        for ((sport_type, round, num), events) in map {
            let json_events = serde_json::to_string(&events).unwrap();
            let key = (sport_type.to_string(), *round, *num);
            debug!("key = {:?}, value = {}", key, json_events);
            static_map.insert(key, json_events);
        }
    }
}
