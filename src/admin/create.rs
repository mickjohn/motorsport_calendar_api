use super::super::model;
use super::super::database;
use rocket::request::Form;
use rocket::response::content;
use diesel::prelude::*;
use diesel::insert_into;
use chrono::NaiveDateTime;

fn return_link(url: &str) -> String {
    format!(r#"<!DOCTYPE html>
              <html>
              <head>
              <style>
              table, th, td {{
                border: 1px solid black;
                border-collapse: collapse;
              }}
              </style>
              <link rel="stylesheet" href="/static/stylesheet.css">
              </head>
              <body>
                  <a href="{}">Back to event</a>
              </body>
              </html>"#, url).to_string()
    }


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

#[post("/events/<e_id>/create_session", data="<new_session_form>")]
fn create_session(e_id: i32, new_session_form: Form<NewSessionForm>) -> content::Html<String> {
    use super::super::schema::sessions::dsl::*;
    let new_session = new_session_form.into_inner();
    let session_model = model::NewSession {
        name: new_session.name,
        date: convert_date_string(&new_session.date),
        time: convert_date_string(&new_session.time),
        event_id: new_session.event_id,

    };
    let connection = database::establish_connection();

    insert_into(sessions)
        .values(&session_model)
        .execute(&connection)
        .expect("Failed to insert session");

    // "Session Added :D".to_string()
    let url = format!("/events/{}", e_id);
    content::Html(return_link(&url))
    // format!(RETURN_LINK, e_id).to_string()
}

#[post("/events/create_event", data="<new_event_form>")]
fn create_event(new_event_form: Form<NewEventForm>) -> String {
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

    "Event Created! :)".to_string()
}
