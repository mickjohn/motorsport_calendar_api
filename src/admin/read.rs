use super::super::database;
use super::super::model::{Event as MEvent, Session as MSession};
use super::super::schema::*;
use diesel::prelude::*;
use rocket_contrib::Template;
use tera::Context;

#[get("/events")]
pub fn get_events() -> Template {
    let connection = database::establish_connection();
    let events: Vec<MEvent> = events::table
        .load(&connection)
        .expect("Error loading events");

    let mut context = Context::new();
    context.add("events", &events);
    Template::render("events", &context)
}

#[get("/events/<event_id>")]
fn get_event(event_id: i32) -> Template {
    let connection = database::establish_connection();
    let event: MEvent = events::table
        .filter(events::id.eq(event_id))
        .first(&connection)
        .expect("Error loading event");
    let sessions: Vec<MSession> = sessions::table
        .filter(sessions::event_id.eq(event.id))
        .load(&connection)
        .expect("Error lading sessions");

    let mut context = Context::new();
    context.add("event", &event);
    context.add("sessions", &sessions);
    Template::render("event", &context)
}

#[get("/sessions/<session_id>")]
fn get_session(session_id: i32) -> Template {
    let connection = database::establish_connection();
    let session: MSession = sessions::table
        .filter(sessions::id.eq(session_id))
        .first(&connection)
        .expect("Error loading event");

    let mut context = Context::new();
    context.add("session", &session);
    Template::render("session", &context)
}

#[derive(FromForm)]
struct CreateSessionFromParams {
    session_name: Option<String>,
    date_string: Option<String>,
}

#[get("/events/<event_id>/create_session?<params>")]
fn create_session_form(event_id: i32, params: CreateSessionFromParams) -> Template {
    let mut context = Context::new();

    if let (Some(session_name), Some(date_string)) = (params.session_name, params.date_string) {
        context.add("session_name", &session_name);
        context.add("date_string", &date_string);
    }

    context.add("event_id", &event_id);
    Template::render("create_session", &context)
}

#[get("/events/<event_id>/create_session")]
fn create_session_form_no_params(event_id: i32) -> Template {
    let mut context = Context::new();
    context.add("event_id", &event_id);
    Template::render("create_session", &context)
}

#[get("/events/create_event")]
fn create_event_form() -> Template {
    let context = Context::new();
    Template::render("create_event", &context)
}

#[get("/events/<sport_type>/create_event")]
fn create_event_form_for_sport_type(sport_type: String) -> Template {
    let mut context = Context::new();
    context.add("sport_type", &sport_type);
    Template::render("create_event", &context)
}
