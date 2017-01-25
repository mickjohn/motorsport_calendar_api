
use chrono::{DateTime, UTC};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Event {
    pub sport: String,
    pub round: u64,
    pub country: String,
    pub location: String,
    pub sessions: Vec<Session>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Session {
   pub name: String,
   pub date: String,
   pub time: u64,
}
