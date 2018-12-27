use super::super::schema::*;

#[cfg(test)]
use std::convert::From;

#[derive(PartialEq, Debug, Clone, Queryable, Insertable, Identifiable, AsChangeset, Associations,
         Serialize)]
#[table_name = "events"]
pub struct Event {
    pub id: i32,
    pub sport: String,
    pub title: String,
    pub country: String,
    pub location: String,
    pub track: String,
}

#[derive(Debug, Insertable, FromForm, Serialize, Deserialize)]
#[table_name = "events"]
pub struct NewEvent {
    pub sport: String,
    pub title: String,
    pub country: String,
    pub location: String,
    pub track: String,
}

#[derive(Debug, Insertable, FromForm, Deserialize, Serialize)]
#[table_name = "events"]
pub struct UpdateEvent {
    pub sport: String,
    pub title: String,
    pub country: String,
    pub location: String,
    pub track: String,
}

#[cfg(test)] // Only used in test code
impl From<Event> for UpdateEvent {
    fn from(new_event: Event) -> Self {
        UpdateEvent {
            sport: new_event.sport,
            title: new_event.title,
            country: new_event.country,
            location: new_event.location,
            track: new_event.track,
        }
    }
}
