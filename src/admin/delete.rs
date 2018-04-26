use super::super::database;
use super::super::schema::*;
use diesel;
use diesel::prelude::*;

#[delete("/events/<e_id>")]
fn delete_event(e_id: i32) {
    let connection = database::establish_connection();
    diesel::delete(events::table.filter(events::id.eq(&e_id)))
        .execute(&connection)
        .expect("Error deleting event");
}

// Returns nothing, the webpage is expected to reload the page.
#[delete("/sessions/<e_id>")]
fn delete_session(e_id: i32) {
    let connection = database::establish_connection();
    diesel::delete(sessions::table.filter(sessions::id.eq(&e_id)))
        .execute(&connection)
        .expect("Error deleting event");
}
