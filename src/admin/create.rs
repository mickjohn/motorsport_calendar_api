use super::super::model;
use super::super::model::{Event as MEvent, NewSession};
use super::super::database;
use super::super::schema::*;
use rocket_contrib::Template;
use rocket::request::Form;
use diesel::prelude::*;
use diesel::insert_into;
use chrono::{NaiveDateTime, NaiveDate};
use tera::Context;

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

#[post("/events/<e_id>/create_session", data="<new_session_form>")]
fn create_session(e_id: i32, new_session_form: Form<NewSessionForm>) -> Template {
    use super::super::schema::sessions::dsl::*;
    let new_session = new_session_form.into_inner();
    let session_model = model::NewSession {
        name: new_session.name,
        date: convert_date_string(&new_session.time).map(|d| d.date().and_hms(0,0,0)),
        time: convert_date_string(&new_session.time),
        event_id: new_session.event_id,
    };

    let connection = database::establish_connection();
    let event: MEvent = events::table.filter(events::id.eq(e_id)).first(&connection).expect("Error loading event");

    insert_into(sessions)
        .values(&session_model)
        .execute(&connection)
        .expect("Failed to insert session");

    let mut context = Context::new();
    context.add("event_id", &e_id);
    // Prepare some parameters to autofill the next forms
    if let Some((d, session_name)) = derive_next_session_link(&event.sport, &session_model) {
        let date_string = d.format("%Y-%m-%dT00:00:00").to_string();
        context.add("session_name", &session_name);
        context.add("date_string", &date_string)
    }
    Template::render("created_session", &context)
}

#[post("/events/create_event", data="<new_event_form>")]
fn create_event(new_event_form: Form<NewEventForm>) -> Template {
    use super::super::schema::events::dsl::*;
    let new_event = new_event_form.into_inner();
    let event_model = model::NewEvent {
        sport: new_event.sport,
        round: new_event.round,
        country: new_event.country,
        location: new_event.location,
    };
    let connection = database::establish_connection();

    insert_into(events)
        .values(&event_model)
        .execute(&connection)
        .expect("Failed to insert new event");

    let context = Context::new();
    Template::render("created_event", &context)
}

// event_type -> Formula 1
// new_session -> NewSession { name: "Practice 2", date: Some(2018-05-11T00:00:00), time: Some(2018-05-11T14:00:00), event_id: 5 }

fn derive_next_session_link(event_type: &str, new_session: &NewSession) -> Option<(NaiveDate, String)> {
    match event_type {
        "Formula 1" => {
            match new_session.name.as_str() {
                "Practice 1" => new_session.date.map(|d| (d.date(), "Practice 2".to_string())),
                "Practice 2" => new_session.date.map(|d| (d.date().succ(), "Practice 3".to_string())),
                "Practice 3" => new_session.date.map(|d| (d.date(), "Qualifying".to_string())),
                "Qualifying" => new_session.date.map(|d| (d.date().succ(), "Race".to_string())),
                _ => None,
            }
        },
        _ => None,
    }
}
