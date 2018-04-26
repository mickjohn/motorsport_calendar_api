use super::super::auth;
use super::super::database;
use super::super::model::events as event_models;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use rocket_contrib::Json;

#[derive(Debug, PartialEq, Fail)]
pub enum PostError {
    #[fail(display = "Database Error")]
    DieselError(DieselError),
}

#[get("/login")]
fn login(user: auth::UserWithPlaintextPassword) -> String {
    let connection = database::establish_connection();
    match auth::validate_user(&user, &connection) {
        Ok(_) => "Success!".to_string(),
        Err(_) => ">:(".to_string(),
    }
}

#[post("/create_event", data = "<new_event_json>")]
fn create_event(
    user: auth::UserWithPlaintextPassword,
    new_event_json: Json<event_models::NewEvent>,
) -> String {
    let new_event = new_event_json.into_inner();
    let connection = database::establish_connection();
    match auth::validate_user(&user, &connection) {
        Ok(_) => {
            match insert_new_event(&connection, &new_event) {
                Ok(_) => "Successfully created new event".to_string(),
                Err(_) => "Encountered database error inserting new event".to_string(),
            }
        },
        Err(_) => "Failed to authenticate. Wrong username or password".to_string(),
    }
}

fn insert_new_event(
    db_conn: &SqliteConnection,
    new_event: &event_models::NewEvent,
) -> Result<usize, PostError> {
    use super::super::schema::events::dsl::*;
    insert_into(events)
        .values(new_event)
        .execute(db_conn)
        .map_err(|e| PostError::DieselError(e))
        .map(|u| Ok(u))?
}
