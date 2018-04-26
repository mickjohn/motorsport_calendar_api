use super::super::schema::*;
use super::events::Event;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Queryable, Insertable, AsChangeset, Associations, Identifiable, Serialize)]
#[belongs_to(Event, foreign_key = "event_id")]
#[table_name = "sessions"]
pub struct Session {
    pub id: i32,
    pub name: String,
    pub date: Option<NaiveDateTime>,
    pub time: Option<NaiveDateTime>,
    pub event_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "sessions"]
pub struct NewSession {
    pub name: String,
    pub date: Option<NaiveDateTime>,
    pub time: Option<NaiveDateTime>,
    pub event_id: i32,
}
