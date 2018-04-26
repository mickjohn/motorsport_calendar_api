use super::database;
use diesel::prelude::*;
use rocket;
use rocket::response::content;
use rocket::response::status::NotFound;
use rocket::Rocket;
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

// #[cfg(test)]
// mod test {
//     use super::init_rocket;
//     use super::super::model;
//     use super::super::schema::{events, sessions};
//     use super::super::model::{Event as MEvent, Session as MSession};
//     use super::super::test_functions::EventGeneratorBuilder;

//     use std::fs;
//     use diesel;
//     use diesel::prelude::*;
//     use rocket::local::Client;
//     use rocket::http::Status;
//     use rusqlite::Connection as RusqliteConnection;
//     use motorsport_calendar_common::event::Event as CEvent;
//     use motorsport_calendar_common::event::Session as CSession;
//     use serde_json;

//     // const SPORTS: &'static [&'static str] = &["Formula 1", "Indycar", "DTM", "Formula 2", "Formula 3", "GP3"];
//     const SPORTS: &'static [&'static str] = &["Formula 1"];
//     const LOCATIONS: &'static [(&'static str, &'static str)] = &[
//         ("Australia", "Albert Park"),
//         ("Bahrain", "Bahrain"),
//         ("Shanghai", "Shanghai"),
//         ("Azerbaijan", "Baku"),
//         ("Spain", "Catalunya"),
//         ("Monaco", "Monaco"),
//         ("Canada", "Circuit Gilles Villeneuve"),
//         ("France", "Circuit Paul Ricard"),
//         ("Austria", "Red Bull Ring"),
//     ];

//     fn get_table_sql() -> (String, String) {
//         (
//             include_str!("../migrations/20171019211358_events/up.sql").to_string(),
//             include_str!("../migrations/20171019211407_sessions/up.sql").to_string(),
//         )
//     }

//     fn run_sql_string(conn: &RusqliteConnection, s: &str) {
//         for sql in s.split(";") {
//             let trimmed = sql.trim();
//             if !trimmed.starts_with("/*") && !trimmed.starts_with("*/") && trimmed != "" {
//                 conn.execute(trimmed, &[]).unwrap();
//             }
//         }
//     }

//     fn generate_db_name() -> String {
//         use rand::{thread_rng, Rng};
//         let s: String = thread_rng().gen_ascii_chars().take(30).collect();
//         format!("sqlite/test/{}.db", s)
//     }

//     fn generate_test_events() -> Vec<CEvent> {
//         let mut event_id = 0;
//         let mut session_id = 0;
//         let mut events = Vec::new();

//         let locations: Vec<(String, String)> = LOCATIONS
//             .iter()
//             .map(|&(l, c)| (l.to_string(), c.to_string()))
//             .collect();

//         for sport in SPORTS {
//             let mut generator = EventGeneratorBuilder::with()
//                 .number(40)
//                 .sport(sport.to_string())
//                 .locations(locations.clone())
//                 .sessions(vec![5])
//                 .starting_id(event_id)
//                 .starting_session_id(session_id)
//                 .finish();
//             let (new_events, new_e_id, new_s_id) = generator.generate_events();
//             events.extend(new_events);
//             event_id = new_e_id;
//             session_id = new_s_id;
//         }
//         events
//     }

//     fn insert_test_data(url: &str, events: Vec<CEvent>) {
//         let d_conn = SqliteConnection::establish(url).unwrap();
//         for e in events {
//             let (model_event, model_sessions) = model::into_models(e.clone());
//             diesel::insert_into(events::table)
//                 .values(&model_event)
//                 .execute(&d_conn)
//                 .expect("Error inserting event");

//             diesel::insert_into(sessions::table)
//                 .values(&model_sessions)
//                 .execute(&d_conn)
//                 .expect("Error inserting event");
//         }
//     }

//     fn create_database(url: &str) {
//         // delete_database_if_exists(url); // clear DB from last run
//         let conn = RusqliteConnection::open(url).unwrap();
//         let (event_sql, session_sql) = get_table_sql();
//         run_sql_string(&conn, &event_sql);
//         run_sql_string(&conn, &session_sql);
//     }

//     fn delete_database_if_exists(url: &str) {
//         fs::remove_file(url);
//     }

//     describe! api {
//         before_each {
//             let db_url = generate_db_name();
//             let events = generate_test_events();
//             create_database(&db_url);
//             insert_test_data(&db_url, events.clone());
//             let d_conn = SqliteConnection::establish(&db_url).unwrap();
//             let client =
//                 Client::new(init_rocket(Some(&db_url))).expect("valid rocket instance");
//         }

//         it "database should have events" {
//             let mevents = events::table.load::<MEvent>(&d_conn).unwrap();
//             assert!(mevents.len() > 0);
//         }

//         it "returns all of the events in the database" {
//             let mut response = client.get("/events").dispatch();
//             assert_eq!(response.status(), Status::Ok);
//             let mut response_events: Vec<CEvent> =
//                 serde_json::from_str(&response.body_string().unwrap()).unwrap();
//             assert_eq!(events, response_events );
//         }

//         it "can return single event" {
//             let mut response = client.get("/events/1").dispatch();
//             assert_eq!(response.status(), Status::Ok);
//             let mut response_event: CEvent =
//                 serde_json::from_str(&response.body_string().unwrap()).unwrap();
//             assert_eq!(events[0], response_event);
//         }

//         it "returns 404 when event doesn't exist" {
//             let mut response = client.get("/events/0").dispatch();
//             assert_eq!(response.status(), Status::NotFound);
//         }

//         after_each {
//             delete_database_if_exists(&db_url);
//         }
//     }
// }
