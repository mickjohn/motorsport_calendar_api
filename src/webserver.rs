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
    use super::super::test_functions::EventGeneratorBuilder;
    use rusqlite::Connection as RusqliteConnection;
    use diesel;
    use diesel::prelude::*;
    use std::fs;

    fn get_table_sql() -> (String, String) {
        (include_str!("../migrations/20171019211358_events/up.sql").to_string()
         , include_str!("../migrations/20171019211407_sessions/up.sql").to_string())
    }

    fn run_sql_string(conn: &RusqliteConnection, s: &str) {
        for sql in s.split(";") {
            let trimmed = sql.trim();
            if !trimmed.starts_with("/*") && !trimmed.starts_with("*/") && trimmed != "" {
                conn.execute(trimmed, &[]).unwrap();
            }
        }
    }

    fn mick(url: &str) {
        let mut event_id = 0;
        let mut session_id = 0;
        let mut events = Vec::new();
        let sports = vec!["Formula 1", "Indycar", "DTM", "Formula 2", "GP3", "Formula 3"];
        let locations: Vec<(String, String)> = vec![("Australia", "Albert Park"),
        ("Bahrain", "Bahrain"),
        ("Shanghai", "Shanghai"),
        ("Azerbaijan", "Baku"),
        ("Spain", "Catalunya"),
        ("Monaco", "Monaco"),
        ("Canada", "Circuit Gilles Villeneuve"),
        ("France", "Circuit Paul Ricard"),
        ("Austria", "Red Bull Ring"),
        ].iter()
            .map(|&(l,c)| (l.to_string(), c.to_string()))
            .collect();

        for sport in sports {
            let mut generator = EventGeneratorBuilder::with()
                .number(40)
                .sport(sport.to_string())
                .locations(locations.clone())
                .sessions(vec![5])
                .starting_id(event_id)
                .starting_session_id(session_id)
                .finish();
            let (new_events, new_e_id, new_s_id) = generator.generate_events();
            events.extend(new_events);
            event_id = new_e_id;
            session_id = new_s_id;
        }

        let d_conn = SqliteConnection::establish(url).unwrap();
        for e in events {
            let (model_event, model_sessions) = model::into_models(e.clone());
            diesel::insert_into(events::table)
                .values(&model_event)
                .execute(&d_conn)
                .expect("Error inserting event");

            diesel::insert_into(sessions::table)
                .values(&model_sessions)
                .execute(&d_conn)
                .expect("Error inserting event");
        }
    }

    fn create_database(url: &str) {
        delete_database(url); // clear DB from last run
        let conn = RusqliteConnection::open(url).unwrap();
        let (event_sql, session_sql) = get_table_sql();
        run_sql_string(&conn, &event_sql);
        run_sql_string(&conn, &session_sql);
        mick(url);
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

        it "database should have events" {
            let mevents = events::table.load::<MEvent>(&d_conn).unwrap();
            assert!(mevents.len() > 0);
        }

        // after_each {
        //     delete_database(&db_url);
        // }
    }
}
