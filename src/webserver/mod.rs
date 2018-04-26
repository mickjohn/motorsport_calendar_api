use super::database;
use super::model;
use diesel::prelude::*;
use rocket;
use rocket::Request;
use rocket::Rocket;
use std::sync::Mutex;

mod post;
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
            routes![read::all_events, read::event, post::create_event],
        )
        .mount("/", routes![post::login])
        .catch(errors![bad_request])
        .manage(pool)
}

pub fn start(option_url: Option<&str>) {
    init_rocket(option_url).launch();
}

#[error(400)]
fn bad_request(_req: &Request) -> String {
    "400 - Bad Request. Double check that the syntax of the request is correct.".to_string()
}
