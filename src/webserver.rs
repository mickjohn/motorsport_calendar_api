use rocket;
use rocket::response::content;
use rocket::response::status::NotFound;
use rocket::State;
use diesel::prelude::*;
use super::database;

// Event and session models
use super::model::Event as MEvent;
use super::model::Session as MSession;
use schema::events;
use schema::sessions;
use super::model;
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
fn event(conn_pool: State<Mutex<SqliteConnection>>, event_id: i32) 
    -> Result<content::Json<String>, NotFound<String>> {
    use diesel::result::Error::NotFound as DbNotFound;
    let ref conn = *conn_pool.lock().unwrap();
    let conn = database::establish_connection();
    let model_event_result = events::table
        .filter(events::id.eq(event_id))
        .first::<MEvent>(&conn);

    match model_event_result {
        Ok(model_event) => {
            let model_sessions: Vec<MSession> = sessions::table
                .filter(sessions::event_id.eq(&event_id))
                .load(&conn)
                .expect("Can't load sessions");

            let e = model::from_model(model_event, model_sessions);
            let json = serde_json::to_string(&e).unwrap();
            Ok(content::Json(json))
        },
        Err(DbNotFound) => Err(NotFound("Could not find resource".to_string())),
        _ => panic!("Couldn't load events!")
    }

    // let model_sessions: Vec<MSession> = sessions::table
    //     .filter(sessions::event_id.eq(&event_id))
    //     .load(&conn)
    //     .expect("Can't load sessions");

    // let e = model::from_model(model_event, model_sessions);
    // let json = serde_json::to_string(&e).unwrap();
    // Ok(content::Json(json))
    }

fn get_db_pool() -> Mutex<SqliteConnection> {
    let conn = database::establish_connection();
    Mutex::new(conn)
}

pub fn start() {
    let pool = get_db_pool();
    rocket::ignite()
        .mount("/events", routes![
               all_events,
               event,
        ])
        .manage(pool)
        .launch();
}

#[cfg(test)]
mod test {
    use super::super::model;
    use super::super::schema::{events, sessions};
    use super::super::model::{Event as MEvent, Session as MSession};
    use rusqlite::Connection as RusqliteConnection;
    use diesel::prelude::*;
    use std::fs;

    fn get_table_sql() -> (String, String) {
        (include_str!("../migrations/20171019211358_events/up.sql").to_string()
         , include_str!("../migrations/20171019211407_sessions/up.sql").to_string())
    }

    fn create_database(url: &str) {
        let conn = RusqliteConnection::open(url).unwrap();
        let (event_sql, session_sql) = get_table_sql();
        for sql in event_sql.split(";") {
            let trimmed = sql.trim();
            if !trimmed.starts_with("/*") && !trimmed.starts_with("*/") && trimmed != "" {
                conn.execute(trimmed, &[]).unwrap();
            }
        }

        for sql in session_sql.split(";") {
            let trimmed = sql.trim();
            if !trimmed.starts_with("/*") && !trimmed.starts_with("*/") && trimmed != "" {
                conn.execute(trimmed, &[]).unwrap();
            }
        }
    }

    fn delete_database(url: &str) {
        fs::remove_file(url).unwrap();
    }

    describe! stainless {
        before_each {
            let db_url = "sqlite/tempdatabase.db";
            create_database(&db_url);
            let d_conn = SqliteConnection::establish(&db_url).unwrap();
        }

        it "Database has events" {
            let mevents = events::table.load::<MEvent>(&d_conn).unwrap();
            assert!(mevents.len() > 0);
        }

        after_each {
            delete_database(&db_url);
        }
    }
}

// #[cfg(test)]
// mod test {
//     use rocket::testing::MockRequest;
//     use rocket::http::{Status, Method, ContentType};
//     use rocket::{Response, Rocket};
//     use serde_json;
//     use diesel::prelude::*;
//     use chrono::{NaiveDate, NaiveDateTime};
//     // use motorsport_calendar_common::event::Event;
//     use super::*;
//     use super::super::model::Event as MEvent;
//     use super::super::model::Sessions as MSession;
//     use super::super::event_loader;
//     use super::super::data::json_data;
//     use std::io::prelude::*;
//     use std::fs::File;

//     // const TEST_DATA_DIR: &'static str = "test/data";

//     fn init_test_data() -> Vec<Event> {
//         let events = event_loader::load_events_from_yml_file(&format!("{}/test_data.yml", TEST_DATA_DIR)).unwrap();
//         json_data::init(&events);
//         events
//     }

//     // fn init_db(events: Vec<Event>) -> SqliteConnection {
//     // }


//     fn get_test_data() -> (Vec<MEvent>, Vec<MSession>) {
//         (load_test_events(), load_test_sessions())
//     }

//     fn load_test_events() -> Vec<MEvent> {
//         let mut f = File::open("test/data/test_events.json").unwrap();
//         let mut s = String::new();
//         f.read_to_string(&mut s).unwrap();
//         serde_json::from_str(&s).unwrap()
//     }

//     fn load_test_sessions() -> Vec<MSession> {
//         let mut f = File::open("test/data/test_sessions.json").unwrap();
//         let mut s = String::new();
//         f.read_to_string(&mut s).unwrap();
//         serde_json::from_str(&s).unwrap()
//     }

//     fn init_rocket() -> Rocket {
//         rocket::ignite()
//             .mount("/", routes![
//                    events,
//                    event,
//                    ])
//     }

//     fn assert_status_and_string(mut response: Response,status: Status, s: String) {
//         assert_eq!(response.status(), status);
//         let body_string = response.body().and_then(|b| b.into_string()).unwrap();
//         assert_eq!(body_string, s);
//     }

//     #[test]
//     fn test_get_events() {
//         let test_events = init_test_data();
//         let rocket = init_rocket();
//         let mut req = MockRequest::new(Method::Get, "/events").header(ContentType::Json);
//         let response = req.dispatch_with(&rocket);
//         let expected = serde_json::to_string(&test_events).unwrap();
//         assert_status_and_string(response, Status::Ok, expected);
//     }

//     #[test]
//     fn test_get_events_by_type() {
//         init_test_data();
//         let rocket = init_rocket();
//         for sport_type in vec!["DTM", "GP2", "Formula 1", "doesnt_exist"] {
//             let get = format!("/events/{}", sport_type);
//             let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::Json);
//             let response = req.dispatch_with(&rocket);
//             let expected = match json_data::DATA.read().unwrap().events_type.get(sport_type) {
//                 None => "{}".to_string(),
//                 Some(x) => x.to_string()
//             };
//             assert_status_and_string(response, Status::Ok, expected);
//         }
//     }


//     #[test]
//     fn test_get_events_by_type_and_round() {
//         init_test_data();
//         let rocket = init_rocket();
//         let keys = vec![
//             ("DTM".to_string(), 0),
//             ("DTM".to_string(), 1),
//             ("DTM".to_string(), 2),
//             ("DTM".to_string(), 3),
//             ("GP2".to_string(), 999),
//             ("GP2".to_string(), 1),
//             ("GP2".to_string(), 2),
//             ("Formula 1".to_string(), 0),
//             ("Formula 1".to_string(), 1),
//             ];

//         for key in keys {
//             let get = format!("/events/{}/{}", key.0, key.1);
//             let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::Json);
//             let response = req.dispatch_with(&rocket);
//             let expected = match json_data::DATA.read().unwrap().events_type_round.get(&key) {
//                 None => "{}".to_string(),
//                 Some(x) => x.to_string()
//             };
//             assert_status_and_string(response, Status::Ok, expected);
//         }
//     }

//     #[test]
//     fn test_get_events_by_type_and_round_and_session() {
//         init_test_data();
//         let rocket = init_rocket();
//         let keys = vec![
//             ("DTM".to_string(), 0, 1),
//             ("DTM".to_string(), 0, 2),
//             ("DTM".to_string(), 1, 0),
//             ("DTM".to_string(), 1, 1),
//             ("DTM".to_string(), 1, 2),
//             ("DTM".to_string(), 1, 3),
//             ("DTM".to_string(), 2, 1),
//             ("DTM".to_string(), 2, 2),
//             ("DTM".to_string(), 2, 30),
//             ("Formula 1".to_string(), 0, 1),
//             ("Formula 1".to_string(), 1, 2),
//             ("Formula 1".to_string(), 1, 3),
//             ("Formula 1".to_string(), 1, 444),
//             ];

//         for key in keys {
//             let get = format!("/events/{}/{}/{}", key.0, key.1, key.2);
//             let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::Json);
//             let response = req.dispatch_with(&rocket);
//             let expected = match json_data::DATA.read().unwrap().events_type_round_num.get(&key) {
//                 None => "{}".to_string(),
//                 Some(x) => x.to_string()
//             };
//             assert_status_and_string(response, Status::Ok, expected);
//         }
//     }
// }
