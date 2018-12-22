use super::super::schema::*;
use super::events::Event;
use chrono::NaiveDateTime;
use std::convert::From;

#[derive(PartialEq, Debug, Clone, Queryable, Insertable, AsChangeset, Associations, Identifiable,
         Serialize)]
#[belongs_to(Event, foreign_key = "event_id")]
#[table_name = "sessions"]
pub struct Session {
    pub id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
    pub event_id: i32,
}

#[derive(Insertable, Clone, Debug, Serialize, Deserialize)]
#[table_name = "sessions"]
pub struct NewSession {
    pub name: String,
    pub time: Option<NaiveDateTime>,
    pub event_id: i32,
}

#[table_name = "sessions"]
#[derive(Insertable, Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSession {
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSessionPlaceholder {
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

impl From<NewSession> for NewSessionPlaceholder {
    fn from(new_session: NewSession) -> Self {
        NewSessionPlaceholder {
            name: new_session.name,
            time: new_session.time,
        }
    }
}
