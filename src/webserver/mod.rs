use super::database;
use super::log;
use super::model;
use auth;
use diesel::prelude::*;
use rocket;
use rocket::http::Status;
use rocket::response::status;
use rocket::Request;
use rocket::Rocket;
use rocket::State;
use std::sync::Mutex;

mod delete;
mod post;
mod put;
mod read;

fn get_db_pool(db_url: Option<&str>) -> Mutex<SqliteConnection> {
    let conn = db_url.map_or_else(
        || database::establish_connection(),
        |s| database::establish_connection_with_url(s),
    );
    Mutex::new(conn)
}

fn init_rocket(option_url: Option<&str>) -> Rocket {
    let pool = get_db_pool(option_url);
    rocket::ignite()
        .mount(
            "/events",
            routes![
                read::all_events,
                read::event,
                read::session,
                read::event_sessions,
                post::create_event,
                post::create_session,
                put::update_event,
                put::update_session,
                delete::delete_event,
                delete::delete_session,
            ],
        )
        .mount("/", routes![authenticate,])
        .catch(errors![bad_request])
        .manage(pool)
}

pub fn start(option_url: Option<&str>) {
    init_rocket(option_url).launch();
}

#[post("/authenticate")]
fn authenticate(
    conn_pool: State<Mutex<SqliteConnection>>,
    user: auth::UserWithPlaintextPassword,
) -> status::Custom<String> {
    let ref connection = *conn_pool.lock().unwrap();
    match authenticate_user(&user, &connection) {
        Ok(()) => status::Custom(Status::Ok, "Successfully authenticated".to_string()),
        Err(e) => e,
    }
}

fn authenticate_user(
    user: &auth::UserWithPlaintextPassword,
    connection: &SqliteConnection,
) -> Result<(), status::Custom<String>> {
    match auth::validate_user(user, connection) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::info!("user: {}, error: {}", user.user_name, e);
            Err(status::Custom(Status::Forbidden, e.to_string()))
        }
    }
}

#[error(400)]
fn bad_request(_req: &Request) -> String {
    "400 - Bad Request. Double check that the syntax of the request is correct.".to_string()
}

#[cfg(test)]
mod test_utils {
    use super::super::model;
    use super::super::schema::{events, sessions};
    use super::super::test_functions::EventGeneratorBuilder;
    use super::init_rocket;

    use diesel;
    use diesel::prelude::*;
    use motorsport_calendar_common::event::Event as CEvent;
    use rocket::local::Client;
    use rusqlite::Connection as RusqliteConnection;
    use std::fs;
    use rocket::http::ContentType;
    use rocket::http::Header;
    use rocket::http::Status;
    use rocket::local::LocalResponse;

    const SPORTS: &'static [&'static str] = &["Formula 1"];
    const LOCATIONS: &'static [(&'static str, &'static str)] = &[
        ("Australia", "Albert Park"),
        ("Bahrain", "Bahrain"),
        ("Shanghai", "Shanghai"),
        ("Azerbaijan", "Baku"),
        ("Spain", "Catalunya"),
        ("Monaco", "Monaco"),
        ("Canada", "Circuit Gilles Villeneuve"),
        ("France", "Circuit Paul Ricard"),
        ("Austria", "Red Bull Ring"),
    ];

    pub const BASIC_HEADER: &'static str = "Basic dGVzdHVzZXI6cXdlcnR5";
    pub const BASIC_HEADER_WRONG_PASS: &'static str = "Basic dGVzdHVzZXI6cXcxMjMxMjNlcnR5";
    pub const BASIC_HEADER_WRONG_USER: &'static str = "Basic dGVzdHVzZXIxMjMxMjM6cXdlcnR5";
    pub const BASIC_HEADER_NO_PASSWORD: &'static str = "Basic dGVzdHVzZXI=";

    fn get_table_sql() -> (String, String, String) {
        (
            include_str!("../../migrations/20171019211358_events/up.sql").to_string(),
            include_str!("../../migrations/20171019211407_sessions/up.sql").to_string(),
            include_str!("../../migrations/2018-04-15-091115_users/up.sql").to_string(),
        )
    }

    fn run_sql_string(conn: &RusqliteConnection, s: &str) {
        for sql in s.split(";") {
            let trimmed = sql.trim();
            if !trimmed.starts_with("/*") && !trimmed.starts_with("*/") && trimmed != "" {
                conn.execute(trimmed, &[]).unwrap();
            }
        }
    }

    fn generate_db_name() -> String {
        use rand::{thread_rng, Rng};
        let s: String = thread_rng().gen_ascii_chars().take(30).collect();
        format!("sqlite/test/{}.db", s)
    }

    pub fn generate_test_events() -> Vec<CEvent> {
        let mut event_id = 0;
        let mut session_id = 0;
        let mut events = Vec::new();

        let locations: Vec<(String, String)> = LOCATIONS
            .iter()
            .map(|&(l, c)| (l.to_string(), c.to_string()))
            .collect();

        for sport in SPORTS {
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
        events
    }

    fn insert_test_data(url: &str, events: Vec<CEvent>) {
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

    fn insert_test_user(url: &str) {
        let conn = RusqliteConnection::open(url).unwrap();
        let sql = "INSERT INTO users (user_name, hashed_password) VALUES ('testuser', '$2y$12$mhl4gqaMHnPrhR6c5r/Dl.AXk/tS/C6qE4QYtU2HqN5EcDgSwi9Cu')";
        conn.execute(sql, &[]).unwrap();
    }

    fn create_database(url: &str) {
        let conn = RusqliteConnection::open(url).unwrap();
        let (event_sql, session_sql, users_sql) = get_table_sql();
        run_sql_string(&conn, &event_sql);
        run_sql_string(&conn, &session_sql);
        run_sql_string(&conn, &users_sql);
    }

    pub fn delete_database_if_exists(url: &str) {
        fs::remove_file(url);
    }

    pub fn setup() -> (String, SqliteConnection, Client, Vec<CEvent>) {
        let db_url = generate_db_name();
        let events = generate_test_events();
        create_database(&db_url);
        insert_test_data(&db_url, events.clone());
        insert_test_user(&db_url);
        let d_conn = SqliteConnection::establish(&db_url).unwrap();
        let client = Client::new(init_rocket(Some(&db_url))).expect("valid rocket instance");
        (db_url.to_string(), d_conn, client, events)
    }

    pub fn setup_empty_db() -> (String, SqliteConnection, Client) {
        let db_url = generate_db_name();
        create_database(&db_url);
        insert_test_user(&db_url);
        let d_conn = SqliteConnection::establish(&db_url).unwrap();
        let client = Client::new(init_rocket(Some(&db_url))).expect("valid rocket instance");
        (db_url.to_string(), d_conn, client)
    }

    fn post_with_auth_header<'a>(
        basic: Header<'static>,
        client: &'a Client,
    ) -> LocalResponse<'a> {
        client
            .post("/authenticate")
            .header(basic)
            .dispatch()
    }

    #[test]
    fn test_authenticate_with_bad_auth() {
        let (db_url, _, client) = setup_empty_db();
        for basic_details in vec![
            BASIC_HEADER_WRONG_PASS,
            BASIC_HEADER_WRONG_USER,
            BASIC_HEADER_NO_PASSWORD,
        ] {
            let basic = Header::new("Authorization", basic_details);
            let post_response = post_with_auth_header(basic, &client);
            assert_eq!(post_response.status(), Status::Forbidden);
        }
        delete_database_if_exists(&db_url);
    }

    #[test]
    fn test_authenticate() {
        let (db_url, _, client) = setup_empty_db();
        let post_response = client
            .post("/authenticate")
            .header(Header::new("Authorization", BASIC_HEADER))
            .dispatch();
        delete_database_if_exists(&db_url);
    }
}
