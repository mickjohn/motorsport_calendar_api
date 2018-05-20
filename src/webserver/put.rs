use super::super::auth;
use super::super::model::events as event_models;
use super::super::model::sessions as session_models;
use super::super::model::Event as MEvent;
use super::super::model::Session as MSession;
use super::authenticate_user;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::Json;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Fail)]
pub enum PutError {
    #[fail(display = "Database Error")]
    DieselError(DieselError),
    #[fail(display = "No event with that ID exists")]
    EventNotFound,
    #[fail(display = "No session with that ID exists")]
    SessionNotFound,
}

#[put("/<event_id>", data = "<event_update_json>")]
fn update_event(
    conn_pool: State<Mutex<SqliteConnection>>,
    event_id: i32,
    user: auth::UserWithPlaintextPassword,
    event_update_json: Json<event_models::UpdateEvent>,
) -> status::Custom<String> {
    let update_event = event_update_json.into_inner();
    let ref connection = *conn_pool.lock().unwrap();
    match authenticate_user(&user, &connection) {
        Ok(()) => match update_event_in_db(&event_id, &connection, update_event) {
            Ok(_) => status::Custom(Status::Ok, "Successfully updated event".to_string()),
            Err(_) => status::Custom(
                Status::InternalServerError,
                "Encountered database error updating event".to_string(),
            ),
        },
        Err(e) => e,
    }
}

#[put("/<_event_id>/sessions/<session_id>", data = "<session_update_json>")]
fn update_session(
    conn_pool: State<Mutex<SqliteConnection>>,
    _event_id: i32,
    session_id: i32,
    user: auth::UserWithPlaintextPassword,
    session_update_json: Json<session_models::UpdateSession>,
) -> status::Custom<String> {
    let update_session = session_update_json.into_inner();
    let ref connection = *conn_pool.lock().unwrap();
    match authenticate_user(&user, &connection) {
        Ok(()) => match update_session_in_db(&session_id, &connection, update_session) {
            Ok(_) => status::Custom(Status::Ok, "Successfully updated session".to_string()),
            Err(_) => status::Custom(
                Status::InternalServerError,
                "Encountered error updating session".to_string(),
            ),
        },
        Err(e) => e,
    }
}

fn update_event_in_db(
    event_id: &i32,
    db_conn: &SqliteConnection,
    update_event: event_models::UpdateEvent,
) -> Result<(), PutError> {
    use super::super::schema::events::dsl::*;
    let event_result: Result<MEvent, DieselError> = events.filter(id.eq(event_id)).first(db_conn);

    match event_result {
        Ok(mut event) => {
            event.sport = update_event.sport;
            event.round = update_event.round;
            event.country = update_event.country;
            event.location = update_event.location;
            let update_result = event.save_changes::<MEvent>(db_conn);
            match update_result {
                Ok(_) => Ok(()),
                Err(e) => Err(PutError::DieselError(e)),
            }
        }
        Err(DieselError::NotFound) => Err(PutError::EventNotFound),
        Err(e) => Err(PutError::DieselError(e)),
    }
}

fn update_session_in_db(
    session_id: &i32,
    db_conn: &SqliteConnection,
    update_session: session_models::UpdateSession,
) -> Result<(), PutError> {
    use super::super::schema::sessions::dsl::*;
    let session_result: Result<MSession, DieselError> =
        sessions.filter(id.eq(session_id)).first(db_conn);

    match session_result {
        Ok(mut session) => {
            session.name = update_session.name;
            session.date = update_session.date;
            session.time = update_session.time;
            let update_result = session.save_changes::<MSession>(db_conn);
            match update_result {
                Ok(_) => Ok(()),
                Err(e) => Err(PutError::DieselError(e)),
            }
        }
        Err(DieselError::NotFound) => Err(PutError::SessionNotFound),
        Err(e) => Err(PutError::DieselError(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::model;
    use super::super::test_utils;

    use chrono::prelude::*;
    use motorsport_calendar_common::event::Event as CEvent;
    use motorsport_calendar_common::event::Session as CSession;
    use rand::{thread_rng, Rng};
    use rocket::http::ContentType;
    use rocket::http::Header;
    use rocket::http::Status;
    use serde_json;

    #[test]
    fn can_update_event() {
        // Setup
        let (db_url, _, client, events) = test_utils::setup();
        let event = &events[0];
        let mut expected_event = event.clone();

        // Generate new values
        let mut rng = thread_rng();
        let new_sport: String = rng.gen_ascii_chars().take(30).collect();
        let new_round: i32 = rng.gen();
        let new_country: String = rng.gen_ascii_chars().take(30).collect();
        let new_location: String = rng.gen_ascii_chars().take(30).collect();

        // Create update to post
        let update_event = model::UpdateEvent {
            sport: new_sport.clone(),
            round: new_round.clone(),
            country: new_country.clone(),
            location: new_location.clone(),
        };

        // Set the values to the new ones
        expected_event.sport = new_sport;
        expected_event.round = new_round;
        expected_event.country = new_country;
        expected_event.location = new_location;

        // Put the updated event
        let endpoint = format!("/events/{}", event.id);
        let mut put_response = client
            .put(endpoint)
            .header(ContentType::JSON)
            .header(Header::new("Authorization", test_utils::BASIC_HEADER))
            .body(serde_json::to_string(&update_event).unwrap())
            .dispatch();

        // Check that the event was updated
        assert_eq!(put_response.status(), Status::Ok);
        assert_eq!(
            put_response.body_string().unwrap(),
            "Successfully updated event".to_string()
        );

        let mut get_response = client.get(format!("/events/{}", event.id)).dispatch();
        let response_event: CEvent =
            serde_json::from_str(&get_response.body_string().unwrap()).unwrap();
        assert_eq!(expected_event, response_event);
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn can_update_session() {
        // Setup
        let (db_url, _, client, events) = test_utils::setup();
        let session = &events[0].sessions[0];
        let mut expected_session = session.clone();

        // Generate new values
        let new_name: String = thread_rng().gen_ascii_chars().take(30).collect();
        let new_date_utc = Utc.ymd(1916, 1, 1).and_hms(9, 10, 11);
        let new_date_naive = NaiveDate::from_ymd(1916, 1, 1).and_hms(9, 10, 11);

        // Create update to post
        let update_session = model::UpdateSession {
            name: new_name.clone(),
            date: Some(new_date_naive.clone()),
            time: None,
        };

        // Set the values to the new ones
        expected_session.name = new_name;
        expected_session.date = new_date_utc;
        expected_session.time = None;

        // Put the updated event
        println!("{:?}", expected_session);
        println!("{:?}", update_session);
        let endpoint = format!("/events/{}/sessions/{}", session.event_id, session.id);
        let mut put_response = client
            .put(endpoint)
            .header(ContentType::JSON)
            .header(Header::new("Authorization", test_utils::BASIC_HEADER))
            .body(serde_json::to_string(&update_session).unwrap())
            .dispatch();

        // Check that the event was updated
        assert_eq!(put_response.status(), Status::Ok);
        assert_eq!(
            put_response.body_string().unwrap(),
            "Successfully updated session".to_string()
        );

        let mut get_response = client
            .get(format!(
                "/events/{}/sessions/{}",
                session.event_id, session.id
            ))
            .dispatch();
        let response_session: CSession =
            serde_json::from_str(&get_response.body_string().unwrap()).unwrap();
        assert_eq!(expected_session, response_session);
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn test_update_event_auth() {
        use self::test_utils::*;
        let (db_url, _, client, _) = test_utils::setup();
        let update_event = model::UpdateEvent {
            sport: "".to_string(),
            round: 1,
            country: "".to_string(),
            location: "".to_string(),
        };
        for basic_details in vec![
            BASIC_HEADER_WRONG_PASS,
            BASIC_HEADER_WRONG_USER,
            BASIC_HEADER_NO_PASSWORD,
        ] {
            let put_response = client
                .put("/events/1")
                .header(ContentType::JSON)
                .header(Header::new("Authorization", basic_details))
                .body(serde_json::to_string(&update_event).unwrap())
                .dispatch();
            assert_eq!(put_response.status(), Status::Forbidden);
        }
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn test_update_session_auth() {
        use self::test_utils::*;
        let (db_url, _, client, _) = test_utils::setup();
        let update_session = model::UpdateSession {
            name: "".to_string(),
            date: None,
            time: None,
        };
        for basic_details in vec![
            BASIC_HEADER_WRONG_PASS,
            BASIC_HEADER_WRONG_USER,
            BASIC_HEADER_NO_PASSWORD,
        ] {
            let put_response = client
                .put("/events/1/sessions/1")
                .header(ContentType::JSON)
                .header(Header::new("Authorization", basic_details))
                .body(serde_json::to_string(&update_session).unwrap())
                .dispatch();
            assert_eq!(put_response.status(), Status::Forbidden);
        }
        test_utils::delete_database_if_exists(&db_url);
    }
}
