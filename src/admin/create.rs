use super::super::schema::*;
use super::super::model::{Event as MEvent, Session as MSession};
use super::super::database;
use super::*;
use rocket::request::{FromForm, Form, FormItems};
use diesel::prelude::*;
use diesel;
use chrono::NaiveDateTime;

#[derive(Insertable, FromForm)]
#[table_name="events"]
struct NewEvent {
    sport: String,
    round: i32,
    country: String,
    location: String,
}

#[derive(Insertable)]
#[table_name="sessions"]
struct NewSession {
    name: String,
    date: Option<NaiveDateTime>,
    time: Option<NaiveDateTime>,
    event_id: i32,
}

impl<'f> FromForm<'f> for NewSession {
    // In practice, we'd use a more descriptive error type.
    type Error = ();

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<NewSession, ()> {
		let mut name = None;
		let mut date = None;
		let mut time = None;
		let mut event_id = None;

		for (key, value) in items {
			match key.as_str() {
				"name" => {
					let decoded = value.url_decode().map_err(|_| ())?;
					name = Some(decoded);
				},
				"date" => {
					let decoded = value.url_decode().map_err(|_| ())?;
					date = convert_date_string(&decoded);
				},
				"time" => {
					let decoded = value.url_decode().map_err(|_| ())?;
					time = convert_date_string(&decoded);
				},
				"event_id" => {
					let decoded = value.url_decode().map_err(|_| ())?;
					event_id = Some(decoded.parse::<i32>().unwrap());
				},
				_ => return Err(())
			}
		}

		Ok(NewSession {
			name: name.unwrap(),
			date: date,
			time: time,
			event_id: event_id.unwrap(),
		})
    }
}

fn convert_date_string(s: &str) -> Option<NaiveDateTime> {
	if s == "" {
		None
	} else {
		Some(NaiveDateTime::parse_from_str(s, super::DATETIME_FORMAT).unwrap())
	}
}

#[post("/events/<event_id>/create_session", data="<new_session_form>")]
fn create_session(event_id: i32, new_session_form: Form<NewSession>) -> String {
    let new_session = new_session_form.into_inner();
    let connection = database::establish_connection();

    diesel::insert(&new_session).into(sessions::table).execute(&connection).expect("Failed to insert session");

    "Session Added :D".to_string()
}

#[post("/events/create_event", data="<new_event_form>")]
fn create_event(new_event_form: Form<NewEvent>) -> String {
    let new_event = new_event_form.into_inner();
    let connection = database::establish_connection();

    diesel::insert(&new_event)
        .into(events::table)
        .execute(&connection)
        .expect("Failed to insert new event");

    "Event Created! :)".to_string()
}
