use motorsport_calendar_common::event::Event as CEvent; //Common event
use motorsport_calendar_common::event::Session as CSession; //Common event

pub mod events;
pub mod sessions;
pub mod users;

pub use self::events::*;
pub use self::sessions::*;
pub use self::users::*;

pub fn from_model(event_model: Event, session_models: Vec<Session>) -> CEvent {
    let sessions = session_models
        .into_iter()
        .map(|s| convert_session(s))
        .collect();
    CEvent {
        id: event_model.id,
        sport: event_model.sport,
        title: event_model.title,
        country: event_model.country,
        location: event_model.location,
        track: event_model.track,
        sessions: sessions,
    }
}

#[cfg(test)] // Only used by test code
pub fn into_models(e: CEvent) -> (Event, Vec<Session>) {
    let csessions = e.sessions;
    let mut sessions = Vec::new();
    let event = Event {
        id: e.id,
        sport: e.sport,
        title: e.title,
        country: e.country,
        location: e.location,
        track: e.track,
    };

    for s in csessions {
        let session = Session {
            id: s.id,
            event_id: e.id,
            name: s.name,
            time: s.time,
        };
        sessions.push(session);
    }
    (event, sessions)
}

#[cfg(test)] // Only used by test code
pub fn into_new_models(e: CEvent) -> (NewEvent, Vec<NewSession>) {
    let csessions = e.sessions;
    let mut sessions = Vec::new();
    let event = NewEvent {
        sport: e.sport,
        title: e.title,
        country: e.country,
        location: e.location,
        track: e.track,
    };

    for s in csessions {
        let session = NewSession {
            event_id: e.id,
            name: s.name,
            time: s.time,
        };
        sessions.push(session);
    }
    (event, sessions)
}

pub fn convert_session(session: Session) -> CSession {
    // let time = if session.time.is_none() {
    //     None
    // } else {
    //     Some(DateTime::<Utc>::from_utc(session.time.unwrap(), Utc))
    // };

    CSession {
        id: session.id,
        event_id: session.event_id,
        name: session.name,
        time: session.time,
    }
}
