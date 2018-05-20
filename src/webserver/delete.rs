use super::super::auth;
use super::authenticate_user;
use diesel;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Fail)]
pub enum DeleteError {
    #[fail(display = "Database Error")]
    DieselError(DieselError),
}

#[delete("/<event_id>")]
fn delete_event(
    conn_pool: State<Mutex<SqliteConnection>>,
    event_id: i32,
    user: auth::UserWithPlaintextPassword,
) -> status::Custom<String> {
    let ref connection = *conn_pool.lock().unwrap();
    match authenticate_user(&user, &connection) {
        Ok(()) => match delete_event_from_db(&event_id, &connection) {
            Ok(_) => status::Custom(Status::Ok, "Successfully deleted event".to_string()),
            Err(_) => status::Custom(
                Status::InternalServerError,
                "Encountered database error deleting event".to_string(),
            ),
        },
        Err(e) => e,
    }
}

#[delete("/<_event_id>/sessions/<session_id>")]
fn delete_session(
    conn_pool: State<Mutex<SqliteConnection>>,
    _event_id: i32,
    session_id: i32,
    user: auth::UserWithPlaintextPassword,
) -> status::Custom<String> {
    let ref connection = *conn_pool.lock().unwrap();
    match authenticate_user(&user, &connection) {
        Ok(()) => match delete_session_from_db(&session_id, &connection) {
            Ok(_) => status::Custom(Status::Ok, "Successfully deleted session".to_string()),
            Err(_) => status::Custom(
                Status::InternalServerError,
                "Encountered error deleting session".to_string(),
            ),
        },
        Err(e) => e,
    }
}

fn delete_event_from_db(event_id: &i32, db_conn: &SqliteConnection) -> Result<(), DeleteError> {
    use super::super::schema::events::dsl::*;
    diesel::delete(events.filter(id.eq(event_id)))
        .execute(db_conn)
        .map_err(|e| DeleteError::DieselError(e))
        .map(|_| ())
}

fn delete_session_from_db(session_id: &i32, db_conn: &SqliteConnection) -> Result<(), DeleteError> {
    use super::super::schema::sessions::dsl::*;
    diesel::delete(sessions.filter(id.eq(session_id)))
        .execute(db_conn)
        .map_err(|e| DeleteError::DieselError(e))
        .map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::super::test_utils;
    use rocket::http::Header;
    use rocket::http::Status;

    #[test]
    fn can_delete_event() {
        // Setup
        let (db_url, _, client, events) = test_utils::setup();
        let event = &events[0];
        let endpoint = format!("/events/{}", event.id);
        let mut delete_response = client
            .delete(endpoint.as_str())
            .header(Header::new("Authorization", test_utils::BASIC_HEADER))
            .dispatch();

        // Check that the event was deleted
        assert_eq!(delete_response.status(), Status::Ok);
        assert_eq!(
            delete_response.body_string().unwrap(),
            "Successfully deleted event".to_string()
        );

        // Check that the event was deleted
        assert_eq!(delete_response.status(), Status::Ok);

        let get_response = client.get(endpoint.as_str()).dispatch();
        assert_eq!(get_response.status(), Status::NotFound);
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn can_delete_session() {
        // Setup
        let (db_url, _, client, events) = test_utils::setup();
        let event = &events[0];
        let session = &events[0].sessions[0];
        let endpoint = format!("/events/{}/sessions/{}", event.id, session.id);
        let mut delete_response = client
            .delete(endpoint.as_str())
            .header(Header::new("Authorization", test_utils::BASIC_HEADER))
            .dispatch();

        // Check that the session was deleted
        assert_eq!(delete_response.status(), Status::Ok);
        assert_eq!(
            delete_response.body_string().unwrap(),
            "Successfully deleted session".to_string()
        );

        // Check that the session was deleted
        assert_eq!(delete_response.status(), Status::Ok);

        let get_response = client.get(endpoint.as_str()).dispatch();
        assert_eq!(get_response.status(), Status::NotFound);
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn test_delete_event_auth() {
        use self::test_utils::*;
        let (db_url, _, client, _) = test_utils::setup();
        for (i, basic_details) in vec![
            BASIC_HEADER_WRONG_PASS,
            BASIC_HEADER_WRONG_USER,
            BASIC_HEADER_NO_PASSWORD,
        ].iter()
            .enumerate()
        {
            let delete_response = client
                .delete(format!("/events/{}", i))
                .header(Header::new("Authorization", *basic_details))
                .dispatch();
            assert_eq!(delete_response.status(), Status::Forbidden);
        }
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn test_delete_session_auth() {
        use self::test_utils::*;
        let (db_url, _, client, _) = test_utils::setup();
        for (i, basic_details) in vec![
            BASIC_HEADER_WRONG_PASS,
            BASIC_HEADER_WRONG_USER,
            BASIC_HEADER_NO_PASSWORD,
        ].iter()
            .enumerate()
        {
            let delete_response = client
                .delete(format!("/events/{}/sessions/{}", i, i))
                .header(Header::new("Authorization", *basic_details))
                .dispatch();
            assert_eq!(delete_response.status(), Status::Forbidden);
        }
        test_utils::delete_database_if_exists(&db_url);
    }
}
