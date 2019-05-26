use super::super::auth;
use super::super::model::events as event_models;
use super::super::model::sessions as session_models;
use super::authenticate_user;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rocket::http::Status;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::Mutex;

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct ReturnMessage {
    message: String,
    id: i32,
}

#[derive(Debug, PartialEq, Fail)]
pub enum PostError {
    #[fail(display = "Database Error")]
    DieselError(DieselError),
}

#[post("/create_event", data = "<new_event_json>")]
pub fn create_event(
    conn_pool: State<Mutex<SqliteConnection>>,
    user: auth::UserWithPlaintextPassword,
    new_event_json: Json<event_models::NewEvent>,
) -> Result<status::Custom<Json<ReturnMessage>>, status::Custom<String>> {
    let new_event = new_event_json.into_inner();
    let ref connection = *conn_pool.lock().unwrap();
    match authenticate_user(&user, &connection) {
        Ok(()) => match insert_new_event(&connection, &new_event) {
            Ok(id) => {
                println!("MICK ---> ID = {}", id);
                let return_data = ReturnMessage {
                    message: "Created new event".to_string(),
                    id: id,
                };
                Ok(status::Custom(Status::Ok, Json(return_data)))
            }
            Err(_) => Err(status::Custom(
                Status::InternalServerError,
                "Encountered database error inserting new event".to_string(),
            )),
        },
        Err(e) => Err(e),
    }
}

#[post("/<event_id>/create_session", data = "<new_session_json>")]
pub fn create_session(
    event_id: i32,
    conn_pool: State<Mutex<SqliteConnection>>,
    user: auth::UserWithPlaintextPassword,
    new_session_json: Json<session_models::NewSessionPlaceholder>,
) -> status::Custom<String> {
    let new_session_placeholder = new_session_json.into_inner();
    let new_session = session_models::NewSession {
        event_id: event_id,
        name: new_session_placeholder.name,
        time: new_session_placeholder.time,
    };
    let ref connection = *conn_pool.lock().unwrap();
    match authenticate_user(&user, &connection) {
        Ok(()) => match insert_new_session(&connection, &new_session) {
            Ok(_) => status::Custom(Status::Ok, "Successfully created new session".to_string()),
            Err(_) => status::Custom(
                Status::InternalServerError,
                "Encountered database error inserting new event".to_string(),
            ),
        },
        Err(e) => e,
    }
}

fn insert_new_event(
    db_conn: &SqliteConnection,
    new_event: &event_models::NewEvent,
) -> Result<i32, PostError> {
    use super::super::schema::events::dsl::*;
    // insert_into(events)
    //     .values(new_event)
    //     .execute(db_conn)
    //     .map_err(|e| PostError::DieselError(e))
    //     .map(|u| Ok(u))?;
    // events.order(id.desc()).first(db_conn);
    // Ok(1000)
    let event = db_conn.transaction::<event_models::Event, DieselError , _>(|| {
        insert_into(events)
            .values(new_event)
            .execute(db_conn)?;
        events.order(id.desc()).first(db_conn)
    }).map_err(|e| PostError::DieselError(e))?;
    Ok(event.id)
}

fn insert_new_session(
    db_conn: &SqliteConnection,
    new_session: &session_models::NewSession,
) -> Result<usize, PostError> {
    use super::super::schema::sessions::dsl::*;
    insert_into(sessions)
        .values(new_session)
        .execute(db_conn)
        .map_err(|e| PostError::DieselError(e))
        .map(|u| Ok(u))?
}

#[cfg(test)]
mod tests {
    use super::super::super::model;
    use super::super::super::model::Event as MEvent;
    use super::super::super::model::NewEvent;
    use super::super::super::model::NewSessionPlaceholder;
    use super::super::test_utils;

    use motorsport_calendar_common::event::Event as CEvent;
    use rocket::http::ContentType;
    use rocket::http::Header;
    use rocket::http::Status;
    use rocket::local::Client;
    use rocket::local::LocalResponse;
    use serde_json;

    fn test_event() -> (NewEvent, MEvent) {
        (
            NewEvent {
                title: "Grand Prix one".to_string(),
                sport: "F1".to_string(),
                country: "a".to_string(),
                location: "b".to_string(),
                track: "c".to_string(),
            },
            MEvent {
                id: 1,
                title: "Grand Prix one".to_string(),
                sport: "F1".to_string(),
                country: "a".to_string(),
                location: "b".to_string(),
                track: "c".to_string(),
            },
        )
    }

    fn post_event_with_auth_header<'a>(
        basic: Header<'static>,
        e: &'a NewEvent,
        client: &'a Client,
    ) -> LocalResponse<'a> {
        client
            .post("/events/create_event")
            .header(ContentType::JSON)
            .header(basic)
            .body(serde_json::to_string(&e).unwrap())
            .dispatch()
    }

    fn post_event<'a>(e: &'a NewEvent, client: &'a Client) -> LocalResponse<'a> {
        let basic = Header::new("Authorization", test_utils::BASIC_HEADER);
        post_event_with_auth_header(basic, e, client)
    }

    fn post_session_with_auth_header<'a>(
        basic: Header<'static>,
        endpoint: &'a str,
        s: &'a NewSessionPlaceholder,
        client: &'a Client,
    ) -> LocalResponse<'a> {
        client
            .post(endpoint)
            .header(ContentType::JSON)
            .header(basic)
            .body(serde_json::to_string(&s).unwrap())
            .dispatch()
    }

    fn post_session<'a>(
        endpoint: &'a str,
        s: &'a NewSessionPlaceholder,
        client: &'a Client,
    ) -> LocalResponse<'a> {
        client
            .post(endpoint)
            .header(ContentType::JSON)
            .header(Header::new("Authorization", test_utils::BASIC_HEADER))
            .body(serde_json::to_string(&s).unwrap())
            .dispatch()
    }

    #[test]
    fn can_insert_events() {
        let events = test_utils::generate_test_events();
        let (new_event, _) = model::into_new_models(events[0].clone());
        let (expected_model_event, _) = model::into_models(events[0].clone());
        let (db_url, _, client) = test_utils::setup_empty_db();

        let mut post_response = post_event(&new_event, &client);

        // Check that the post request was successful
        assert_eq!(post_response.status(), Status::Ok);
        let returned_message: super::ReturnMessage = serde_json::from_str(&post_response.body_string().unwrap()).unwrap();
        assert_eq!(
            returned_message,
            super::ReturnMessage {
                message: "Created new event".to_string(),
                id: 2,
            }
        );

        // Check that we can query the new event
        let mut get_response = client.get("/events/1").dispatch();
        let response_event: CEvent =
            serde_json::from_str(&get_response.body_string().unwrap()).unwrap();
        let (model_event, _) = model::into_models(response_event);
        assert_eq!(expected_model_event, model_event);
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    #[ignore]
    fn can_insert_event_and_session() {
        // Generate events, and prepare our event & session models, and our new event and new
        // session models
        let events = test_utils::generate_test_events();
        let (db_url, _, client) = test_utils::setup_empty_db();

        // Insert each Event
        for event in events {
            let (new_event_model, new_session_models) = model::into_new_models(event.clone());
            let (expected_model_event, expected_model_sessions) = model::into_models(event.clone());
            let event_id = event.id;
            let mut event_post_response = post_event(&new_event_model, &client);

            // Check that the post request was successful
            assert_eq!(event_post_response.status(), Status::Ok);
            let returned_message: super::ReturnMessage = serde_json::from_str(&event_post_response.body_string().unwrap()).unwrap();
            assert_eq!(
                returned_message,
                super::ReturnMessage {
                    message: "Created new event".to_string(),
                    id: event_id,
                }
            );

            // Insert each session
            for new_session_model in new_session_models {
                let new_session_placeholder =
                    model::NewSessionPlaceholder::from(new_session_model.clone());

                let endpoint = format!("/events/{}/create_session", event_id);
                let mut session_post_response =
                    post_session(&endpoint, &new_session_placeholder, &client);
                assert_eq!(session_post_response.status(), Status::Ok);
                assert_eq!(
                    session_post_response.body_string().unwrap(),
                    "Successfully created new session".to_string()
                );
            }

            // Check that the event is there. This will check that both the events and sessions are
            // there
            let mut get_response = client.get(format!("/events/{}", event_id)).dispatch();
            let response_event: CEvent =
                serde_json::from_str(&get_response.body_string().unwrap()).unwrap();
            let (model_event, model_sessions) = model::into_models(response_event);
            assert_eq!(expected_model_event, model_event);
            assert_eq!(expected_model_sessions, model_sessions);
        }
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn test_new_event_auth() {
        let (new_event, _) = test_event();
        let (db_url, _, client) = test_utils::setup_empty_db();
        use self::test_utils::*;
        for basic_details in vec![
            BASIC_HEADER_WRONG_PASS,
            BASIC_HEADER_WRONG_USER,
            BASIC_HEADER_NO_PASSWORD,
        ] {
            let basic = Header::new("Authorization", basic_details);
            let post_response = post_event_with_auth_header(basic, &new_event, &client);
            assert_eq!(post_response.status(), Status::Forbidden);
        }
        test_utils::delete_database_if_exists(&db_url);
    }

    #[test]
    fn test_new_session_auth() {
        let (db_url, _, client, events) = test_utils::setup();
        let (_, new_session_models) = model::into_new_models(events[0].clone());
        let new_session_model = new_session_models[0].clone();
        let new_session_placeholder = model::NewSessionPlaceholder {
            name: new_session_model.name,
            time: new_session_model.time,
        };
        use self::test_utils::*;
        for basic_details in vec![
            BASIC_HEADER_WRONG_PASS,
            BASIC_HEADER_WRONG_USER,
            BASIC_HEADER_NO_PASSWORD,
        ] {
            let mut post_response = post_session_with_auth_header(
                Header::new("Authorization", basic_details),
                "/events/1/create_session",
                &new_session_placeholder,
                &client,
            );
            assert_eq!(post_response.status(), Status::Forbidden);
        }
        test_utils::delete_database_if_exists(&db_url);
    }
}
