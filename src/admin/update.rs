use super::super::schema::*;
use super::super::model::{Event as MEvent, Session as MSession};
use super::super::database;
use super::*;
use rocket::request::Form;
use rocket::response::Redirect;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(FromForm)]
struct EventUpdateForm {
    pub round: i32,
    pub country: String,
    pub location: String,
}

#[derive(FromForm)]
struct SessionUpdateForm {
    pub name: String,
    pub date: String,
    pub time: String,
}

#[post("/events/<event_id>", data="<event_update_form>")]
fn update_event(event_id: i32, event_update_form: Form<EventUpdateForm>) -> String {
    let event_update = event_update_form.into_inner();

    let connection = database::establish_connection();
    let mut event: MEvent = events::table
        .filter(events::id.eq(event_id))
        .first(&connection)
        .expect("Error loading event");

    event.round = event_update.round;
    event.country = event_update.country;
    event.location = event_update.location;
    event.save_changes::<MEvent>(&connection).expect("Could not update event");

    "Event updated!!!".to_string()
}

#[post("/sessions/<session_id>", data="<session_update_form>")]
fn update_session(session_id: i32, session_update_form: Form<SessionUpdateForm>) -> Redirect {
	let session_update = session_update_form.into_inner();
	let connection = database::establish_connection();
	let mut session: MSession = sessions::table
		.filter(sessions::id.eq(session_id))
		.first(&connection)
		.expect("Error loading event");

	let date = if session_update.date == "" {
		None
	} else {
		Some(NaiveDateTime::parse_from_str(&session_update.date, DATETIME_FORMAT).unwrap())
	};

	let time = if session_update.time == "" {
		None
	} else {
		Some(NaiveDateTime::parse_from_str(&session_update.time, DATETIME_FORMAT).unwrap())
	};

	session.name = session_update.name;
	session.date = date;
	session.time = time;
	session.save_changes::<MSession>(&connection).expect("Could not update session");
    Redirect::to(&format!("/sessions/{}", session_id))
}
