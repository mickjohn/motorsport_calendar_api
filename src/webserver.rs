use rocket;
use data::json_data;

#[get("/", format = "application/json")]
fn events() -> String {
    let events = json_data::EVENTS.lock().unwrap();
    events.clone()
}

#[get("/<event_type>", format = "application/json")]
fn event_type(event_type: &str) -> String {
    info!("Query = '{}'", event_type);
    let events = json_data::EVENTS_TYPE.lock().unwrap();
    match events.get(event_type) {
        Some(e) => e.to_string(),
        None => "{}".to_string(),
    }
}

#[get("/<event_type>/<round>", format = "application/json")]
fn event_type_round(event_type: String, round: u64) -> String {
    info!("Query = '({}, {})'", event_type, round);
    let events = json_data::EVENTS_TYPE_ROUND.lock().unwrap();
    match events.get(&(event_type, round)) {
        Some(e) => e.to_string(),
        None => "{}".to_string(),
    }
}

#[get("/<event_type>/<round>/<num>", format = "application/json")]
fn event_type_round_num(event_type: String, round: u64, num: u64) -> String {
    info!("Query = '({}, {}, {})'", event_type, round, num);
    let events = json_data::EVENTS_TYPE_ROUND_NUM.lock().unwrap();
    match events.get(&(event_type, round, num)) {
        Some(e) => e.to_string(),
        None => "{}".to_string(),
    }
}

pub fn start() {
    rocket::ignite()
        .mount("/events", routes![
               events,
               event_type,
               event_type_round,
               event_type_round_num
               ])
        .launch();
}
