use super::super::schema::*;
use super::super::model;
use super::super::database;
use rocket::request::Form;
use diesel;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(FromForm)]
struct NewEventForm {
    sport: String,
    round: i32,
    country: String,
    location: String,
}

#[derive(FromForm)]
struct NewSessionForm {
    name: String,
    date: String,
    time: String,
    event_id: i32,
}

fn convert_date_string(s: &str) -> Option<NaiveDateTime> {
	if s == "" {
		None
	} else {
		Some(NaiveDateTime::parse_from_str(s, super::DATETIME_FORMAT).unwrap())
	}
}

#[post("/events/<event_id>/create_session", data="<new_session_form>")]
fn create_session(event_id: i32, new_session_form: Form<NewSessionForm>) -> String {
    let new_session = new_session_form.into_inner();
    let session_model = model::NewSession {
        name: new_session.name,
        date: convert_date_string(&new_session.date),
        time: convert_date_string(&new_session.time),
        event_id: new_session.event_id,

    };
    let connection = database::establish_connection();

    diesel::insert(&session_model).into(sessions::table).execute(&connection).expect("Failed to insert session");

    "Session Added :D".to_string()
}

#[post("/events/create_event", data="<new_event_form>")]
fn create_event(new_event_form: Form<NewEventForm>) -> String {
    let new_event = new_event_form.into_inner();
    let event_model = model::NewEvent {
        sport: new_event.sport,
        round: new_event.round,
        country: new_event.country,
        location: new_event.location,
    };
    let connection = database::establish_connection();

    diesel::insert(&event_model)
        .into(events::table)
        .execute(&connection)
        .expect("Failed to insert new event");

    "Event Created! :)".to_string()
}
