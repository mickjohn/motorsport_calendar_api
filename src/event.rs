
#[derive(Debug, Clone, PartialEq, RustcDecodable, Serialize)]
pub struct Event {
    pub sport: String,
    pub round: u64,
    pub number_in_round: u64,
    pub event_name: String,
    pub country: String,
    pub track_name: String,
    pub date: String,
    pub start_time: u64
}
