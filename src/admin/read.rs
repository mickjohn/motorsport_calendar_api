use super::super::schema::*;
use super::super::model::{Event as MEvent, Session as MSession};
use super::super::database;
use rocket_contrib::Template;
use tera::Context;
use diesel::prelude::*;

#[get("/")]
pub fn get_sport_types() -> Template {
    let connection = database::establish_connection();
    let sport_types: Vec<String> = events::table.select(events::sport).group_by(events::sport).load(&connection).expect("Error loading events");
    let mut context = Context::new();
    context.add("sports", &sport_types);
    Template::render("sport_types", &context)
}

#[get("/<sport_type>")]
fn get_sport_type_events(sport_type: String) -> Template {
    let s = sport_type.replace("%20", " ");
    let connection = database::establish_connection();
    let events: Vec<MEvent> = events::table.filter(events::sport.eq(&s))
        .load(&connection)
        .expect("Error loading events");

    let mut context = Context::new();
    context.add("events", &events);
    context.add("sport_type", &sport_type);
    Template::render("sport_type", &context)
}

#[get("/events/<event_id>")]
fn get_event(event_id: i32) -> Template {
    let connection = database::establish_connection();
    let event: MEvent = events::table.filter(events::id.eq(Some(event_id))).first(&connection).expect("Error loading event");
    let sessions: Vec<MSession> = sessions::table.filter(sessions::event_id.eq(event.id.unwrap())).load(&connection).expect("Error lading sessions");

    let mut context = Context::new();
    context.add("event", &event);
    context.add("sessions", &sessions);
    Template::render("event", &context)
}

#[get("/sessions/<session_id>")]
fn get_session(session_id: i32) -> Template {
    let connection = database::establish_connection();
    let session: MSession = sessions::table.filter(sessions::id.eq(Some(session_id))).first(&connection).expect("Error loading event");

    let mut context = Context::new();
    context.add("session", &session);
    Template::render("session", &context)
}

#[get("/events/<event_id>/create_session")]
fn create_session_form(event_id: i32) -> Template {
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
