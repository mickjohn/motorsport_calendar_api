use chrono::prelude::*;
use chrono::Duration;
use motorsport_calendar_common::event::Event as CEvent;
use motorsport_calendar_common::event::Session as CSession;
use rand;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

const SESSION_TYPES: [&'static str; 6] = [
    "Practice 1",
    "Practice 2",
    "Qualifying",
    "Race 1",
    "Race 2",
    "Warmup",
];

fn get_random_session_type() -> String {
    SESSION_TYPES.choose(&mut thread_rng())
        .unwrap()
        .to_string()
}

#[derive(Debug, Clone)]
pub struct EventGenerator {
    last_id: i32,
    starting_session_id: i32,
    number_of_events: u32,
    sport: String,
    locations: Vec<(String, String, String, String)>,
    sessions: Vec<u32>,
    start_date: NaiveDateTime,
}

impl EventGenerator {
    pub fn generate_events(&mut self) -> (Vec<CEvent>, i32, i32) {
        let mut events = Vec::new();
        for _ in 0..self.number_of_events {
            events.push(self.generate_event());
        }
        (events, self.last_id, self.starting_session_id)
    }

    fn generate_sessions(&mut self, event_id: i32) -> Vec<CSession> {
        let mut sg = SessionGenerator {
            last_id: self.starting_session_id,
            event_id: event_id,
            number_of_sessions: 5,
            dt: self.start_date.clone(),
        };
        let sessions = sg.generate_sessions();
        // Update the last sessions ID used, and the started date, as
        // both have progressed by generating the sessions
        self.starting_session_id = sg.last_id;
        self.start_date = sg.dt;
        sessions
    }

    fn advance_date(&mut self) {
        let mut rng = thread_rng();
        let time_to_add = Duration::days(rng.gen_range(1, 16));
        let new_dt = self.start_date.checked_add_signed(time_to_add).unwrap();
        self.start_date = new_dt;
    }

    fn generate_event(&mut self) -> CEvent {
        self.last_id += 1;
        let id = self.last_id;
        let mut rng = rand::thread_rng();
        let sessions = self.generate_sessions(id);
        let (country, location, track, title) = {
            let (c, l, tr, ti) = (&self.locations).choose(&mut rng).unwrap().clone();
            (c.clone(), l.clone(), tr.clone(), ti.clone())
        };
        self.advance_date();

        CEvent {
            id: id,
            sport: self.sport.clone(),
            title: title.clone(),
            country: country.clone(),
            location: location.clone(),
            track: track.clone(),
            sessions: sessions,
        }
    }
}

struct SessionGenerator {
    pub last_id: i32,
    pub event_id: i32,
    pub number_of_sessions: i32,
    pub dt: NaiveDateTime,
}

impl SessionGenerator {
    pub fn generate_sessions(&mut self) -> Vec<CSession> {
        let mut sessions = Vec::new();
        for _ in 0..self.number_of_sessions {
            sessions.push(self.generate());
        }
        sessions
    }

    // Add a random number of hours and minutes to the date.
    fn advance_date(&mut self) {
        let mut rng = thread_rng();
        let hours: i64 = rng.gen_range(0, 23);
        let minutes: i64 = rng.gen_range(0, 60);
        let time_to_add = Duration::hours(hours) + Duration::minutes(minutes);
        let new_dt = self.dt.checked_add_signed(time_to_add).unwrap();
        self.dt = new_dt;
    }

    fn generate(&mut self) -> CSession {
        self.last_id += 1;
        self.advance_date();
        let date = self.dt.date().and_hms(0, 0, 0);
        CSession {
            id: self.last_id,
            event_id: self.event_id,
            name: get_random_session_type(),
            time: Some(self.dt.clone()),
        }
    }
}

pub struct EventGeneratorBuilder {
    starting_id: Option<i32>,
    starting_session_id: Option<i32>,
    number_of_events: Option<u32>,
    sport: Option<String>,
    locations: Option<Vec<(String, String, String, String)>>,
    sessions: Option<Vec<u32>>,
    start_date: Option<NaiveDateTime>,
}

impl EventGeneratorBuilder {
    pub fn with() -> EventGeneratorBuilder {
        EventGeneratorBuilder {
            starting_id: None,
            starting_session_id: None,
            number_of_events: None,
            sport: None,
            locations: None,
            sessions: None,
            start_date: None,
        }
    }

    #[allow(dead_code)]
    pub fn starting_id(mut self, i: i32) -> EventGeneratorBuilder {
        self.starting_id = Some(i);
        self
    }

    #[allow(dead_code)]
    pub fn starting_session_id(mut self, i: i32) -> EventGeneratorBuilder {
        self.starting_session_id = Some(i);
        self
    }

    #[allow(dead_code)]
    pub fn number(mut self, n: u32) -> EventGeneratorBuilder {
        self.number_of_events = Some(n);
        self
    }

    #[allow(dead_code)]
    pub fn sport(mut self, s: String) -> EventGeneratorBuilder {
        self.sport = Some(s);
        self
    }

    #[allow(dead_code)]
    pub fn locations(mut self, l: Vec<(String, String, String, String)>) -> EventGeneratorBuilder {
        self.locations = Some(l);
        self
    }

    #[allow(dead_code)]
    pub fn sessions(mut self, s: Vec<u32>) -> EventGeneratorBuilder {
        self.sessions = Some(s);
        self
    }

    #[allow(dead_code)]
    pub fn start_date(mut self, d: NaiveDateTime) -> EventGeneratorBuilder {
        self.start_date = Some(d);
        self
    }

    pub fn finish(self) -> EventGenerator {
        EventGenerator {
            last_id: self.starting_id.unwrap_or(0),
            starting_session_id: self.starting_session_id.unwrap_or(0),
            number_of_events: self.number_of_events.unwrap_or(20),
            sport: self.sport.unwrap_or("Formula 1".to_string()),
            locations: self
                .locations
                .unwrap_or(vec![("Italy".to_string(), "Monza".to_string(), "Monza".to_string(), "Italian Grand Prix".to_string())]),
            sessions: self.sessions.unwrap_or(vec![5]),
            start_date: self
                .start_date
                .unwrap_or(NaiveDate::from_ymd(2018, 3, 18).and_hms(0, 0, 0)),
        }
    }
}
