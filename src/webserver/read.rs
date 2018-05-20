use diesel::prelude::*;
use rocket::response::content;
use rocket::response::status::NotFound;
use rocket::State;

// Event and session models
use super::model;
use super::model::Event as MEvent;
use super::model::Session as MSession;
use schema::events;
use schema::sessions;
use serde_json;
use std::sync::Mutex;

#[get("/")]
fn all_events(conn_pool: State<Mutex<SqliteConnection>>) -> content::Json<String> {
    let ref conn = *conn_pool.lock().unwrap();
    let mevents = events::table.load::<MEvent>(conn).unwrap();
    let msessions = MSession::belonging_to(&mevents)
        .load::<MSession>(conn)
        .unwrap()
        .grouped_by(&mevents);

    let events = mevents
        .into_iter()
        .zip(msessions)
        .map(|(event_model, session_models)| model::from_model(event_model, session_models))
        .collect::<Vec<_>>();

    let json = serde_json::to_string(&events).unwrap();
    content::Json(json)
}

#[get("/<event_id>")]
fn event(
    conn_pool: State<Mutex<SqliteConnection>>,
    event_id: i32,
) -> Result<content::Json<String>, NotFound<String>> {
    use diesel::result::Error::NotFound as DbNotFound;
    let ref conn: SqliteConnection = *conn_pool.lock().unwrap();
    let model_event_result = events::table
        .filter(events::id.eq(event_id))
        .first::<MEvent>(conn);

    match model_event_result {
        Ok(model_event) => {
            let model_sessions: Vec<MSession> = sessions::table
                .filter(sessions::event_id.eq(&event_id))
                .load(conn)
                .expect("Can't load sessions");

            let e = model::from_model(model_event, model_sessions);
            let json = serde_json::to_string(&e).unwrap();
            Ok(content::Json(json))
        }
        Err(DbNotFound) => Err(NotFound("Could not find resource".to_string())),
        _ => panic!("Couldn't load events!"),
    }
}

#[get("/<event_id>/sessions")]
fn event_sessions(
    conn_pool: State<Mutex<SqliteConnection>>,
    event_id: i32,
) -> Result<content::Json<String>, NotFound<String>> {
    use diesel::result::Error::NotFound as DbNotFound;
    let ref conn: SqliteConnection = *conn_pool.lock().unwrap();
    let model_event_result = events::table
        .filter(events::id.eq(event_id))
        .first::<MEvent>(conn);

    match model_event_result {
        Ok(model_event) => {
            let model_sessions: Vec<MSession> = sessions::table
                .filter(sessions::event_id.eq(&event_id))
                .load(conn)
                .expect("Can't load sessions");

            let e = model::from_model(model_event, model_sessions);
            let json = serde_json::to_string(&e.sessions).unwrap();
            Ok(content::Json(json))
        }
        Err(DbNotFound) => Err(NotFound("Could not find resource".to_string())),
        _ => panic!("Couldn't load events!"),
    }
}

#[get("/<_event_id>/sessions/<session_id>")]
fn session(
    conn_pool: State<Mutex<SqliteConnection>>,
    _event_id: i32,
    session_id: i32,
) -> Result<content::Json<String>, NotFound<String>> {
    let ref conn: SqliteConnection = *conn_pool.lock().unwrap();
    let session_model_result = sessions::table
        .filter(sessions::id.eq(session_id))
        .first::<MSession>(conn);

    match session_model_result {
        Ok(model_session) => {
            let s = model::convert_session(model_session);
            let json = serde_json::to_string(&s).unwrap();
            Ok(content::Json(json))
        }
        Err(_) => Err(NotFound("Could not find resource".to_string())),
    }
}

#[cfg(test)]
mod test {
    use super::super::super::model::Event as MEvent;
    use super::super::super::schema::events;
    use super::super::test_utils;

    use diesel::prelude::*;
    use motorsport_calendar_common::event::Event as CEvent;
    use motorsport_calendar_common::event::Session as CSession;
    use rocket::http::Status;
    use serde_json;

    #[test]
    fn database_should_have_events() {
        let (db_url, d_conn, _, _) = test_utils::setup();
        let mevents = events::table.load::<MEvent>(&d_conn).unwrap();
        assert!(mevents.len() > 0);
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn returns_all_of_the_events_in_the_database() {
        let (db_url, _, client, events) = test_utils::setup();
        let mut response = client.get("/events").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_events: Vec<CEvent> =
            serde_json::from_str(&response.body_string().unwrap()).unwrap();
        assert_eq!(events, response_events);
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn can_return_single_event() {
        let (db_url, _, client, events) = test_utils::setup();
        let mut response = client.get("/events/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_event: CEvent =
            serde_json::from_str(&response.body_string().unwrap()).unwrap();
        assert_eq!(events[0], response_event);
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn can_return_sessions_for_event() {
        let (db_url, _, client, events) = test_utils::setup();
        let mut response = client.get("/events/1/sessions").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_sessions: Vec<CSession> =
            serde_json::from_str(&response.body_string().unwrap()).unwrap();
        assert_eq!(events[0].sessions, response_sessions);
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn can_return_session_for_single_event() {
        let (db_url, _, client, events) = test_utils::setup();
        let mut response = client.get("/events/1/sessions/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response_session: CSession =
            serde_json::from_str(&response.body_string().unwrap()).unwrap();
        assert_eq!(events[0].sessions[0], response_session);
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn returns_404_when_event_doesnt_exist() {
        let (db_url, _, client, _) = test_utils::setup();
        let response = client.get("/events/0").dispatch();
        assert_eq!(response.status(), Status::NotFound);
        test_utils::delete_database_if_exists(&db_url);
    }
}
