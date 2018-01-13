use chrono::{DateTime, NaiveDateTime, Utc};
use motorsport_calendar_common::event::Event as CEvent; //Common event
use motorsport_calendar_common::event::Session as CSession; //Common event
use super::schema::*;

#[derive(Debug, Queryable, Identifiable, AsChangeset, Associations, Clone)]
#[table_name="events"]
pub struct Event {
    pub id: i32,
    pub sport: String,
    pub round: i32,
    pub country: String,
    pub location: String,
}

#[derive(Debug, Queryable, Associations, Identifiable, Clone)]
#[belongs_to(Event, foreign_key = "event_id")]
#[table_name="sessions"]
pub struct Session {
    pub id: i32,
    pub name: String,
    pub date: Option<NaiveDateTime>,
    pub time: Option<NaiveDateTime>,
    pub event_id: i32,
}

pub fn from_model(event_model: Event, session_models: Vec<Session>) -> CEvent {
    let sessions = convert_sessions(session_models);
    CEvent {
        id: event_model.id,
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
            id: session.id,
            event_id: session.event_id,
            name: session.name,
            date: date,
            time: time,
        };
        sessions.push(s);
    }
    sessions
}

#[derive(Insertable, FromForm)]
#[table_name="events"]
pub struct NewEvent {
    pub sport: String,
    pub round: i32,
    pub country: String,
    pub location: String,
}

#[derive(Insertable)]
#[table_name="sessions"]
pub struct NewSession {
    pub name: String,
    pub date: Option<NaiveDateTime>,
    pub time: Option<NaiveDateTime>,
    pub event_id: i32,
}
