use chrono::{DateTime, UTC, Local};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Event {
    pub sport: String,
    pub round: u64,
    pub country: String,
    pub location: String,
    pub start_date: DateTime<UTC>,
    pub end_date: DateTime<UTC>,
    pub sessions: Vec<Session>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Session {
   pub name: String,
   pub date: DateTime<UTC>,
   pub time: Option<DateTime<Local>>,
}
