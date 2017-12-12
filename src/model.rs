use chrono::{DateTime, NaiveDateTime, Utc};
use motorsport_calendar_common::event::Event as CEvent; //Common event
use motorsport_calendar_common::event::Session as CSession; //Common event
use super::schema::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Queryable, Identifiable, AsChangeset)]
#[table_name="events"]
pub struct Event {
    pub id: Option<i32>, // ID is required (i.e. can't be null), but because the field is missing 'not null' in sqllite option is required
    pub sport: String,
    pub round: i32,
    pub country: String,
    pub location: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Queryable, Identifiable, AsChangeset)]
#[table_name="sessions"]
pub struct Session {
    pub id: Option<i32>, // ID is required (i.e. can't be null), but because the field is missing 'not null' in sqllite option is required
    pub name: String,
    pub date: Option<NaiveDateTime>,
    pub time: Option<NaiveDateTime>,
    pub event_id: i32,
}

pub fn from_model(event_model: Event, session_models: Vec<Session>) -> CEvent {
    let sessions = convert_sessions(session_models);
    CEvent {
        id: event_model.id.unwrap(),
        sport: event_model.sport,
        round: i64::from(event_model.round),
        country: event_model.country,
        location: event_model.location,
        sessions: sessions,
    }
}

fn convert_sessions(session_models: Vec<Session>) -> Vec<CSession> {
    let mut sessions = Vec::new();
    for session in session_models {
        let date = DateTime::<Utc>::from_utc(session.date.unwrap(), Utc);
        let time = if session.time.is_none() {
            None
        } else {
            Some(DateTime::<Utc>::from_utc(session.time.unwrap(), Utc))
        };


        let s = CSession {
            id: session.id.unwrap(),
            event_id: session.event_id,
            name: session.name,
            date: date,
            time: time,
        };
        sessions.push(s);
    }
    sessions
}
