use chrono::{DateTime, NaiveDateTime, Utc};
use motorsport_calendar_common::event::Event as CEvent; //Common event
use motorsport_calendar_common::event::Session as CSession; //Common event
use super::schema::*;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, AsChangeset, Associations, Serialize)]
#[table_name="events"]
pub struct Event {
    pub id: i32,
    pub sport: String,
    pub round: i32,
    pub country: String,
    pub location: String,
}

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Associations, Identifiable, Serialize)]
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
        round: event_model.round,
        country: event_model.country,
        location: event_model.location,
        sessions: sessions,
    }
}

pub fn into_models(e: CEvent) -> (Event, Vec<Session>) {
    let csessions = e.sessions;
    let mut sessions = Vec::new();
    let event = Event {
        id: e.id,
        sport: e.sport,
        round: e.round,
        country: e.country,
        location: e.location,
    };

    for s in csessions {
        let session = Session {
            id: s.id,
            event_id: e.id,
            name: s.name,
            date: Some(s.date.naive_utc()),
            time: Some(s.time.unwrap().naive_utc()),
        };
        sessions.push(session);
    }
    (event, sessions)
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
