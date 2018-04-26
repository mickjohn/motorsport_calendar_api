use super::super::schema::*;

#[derive(Debug, Clone, Queryable, Insertable, Identifiable, AsChangeset, Associations, Serialize)]
#[table_name = "events"]
pub struct Event {
    pub id: i32,
    pub sport: String,
    pub round: i32,
    pub country: String,
    pub location: String,
}

#[derive(Debug, Insertable, FromForm, Deserialize)]
#[table_name = "events"]
pub struct NewEvent {
    pub sport: String,
    pub round: i32,
    pub country: String,
    pub location: String,
}

#[derive(Debug, Insertable, FromForm, Deserialize)]
#[table_name = "events"]
pub struct UpdateEvent {
    pub sport: String,
    pub round: i32,
    pub country: String,
    pub location: String,
}
