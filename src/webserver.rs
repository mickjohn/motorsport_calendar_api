use rocket;
use data::json_data;

#[get("/", format = "application/json")]
fn events() -> String {
    let data = json_data::DATA.read().unwrap();
    data.events.clone()
}

#[get("/<event_type>", format = "application/json")]
fn event_type(event_type: &str) -> String {
    info!("Query = '{}'", event_type);
    let data = json_data::DATA.read().unwrap();
    let k = event_type.replace("%20", " ");
    match data.events_type.get(&k) {
        Some(e) => e.to_string(),
        None => "{}".to_string(),
    }
}

#[get("/<event_type>/<round>", format = "application/json")]
fn event_type_round(event_type: String, round: u64) -> String {
    info!("Query = '({}, {})'", event_type, round);
    let data = json_data::DATA.read().unwrap();
    match data.events_type_round.get(&(event_type, round)) {
        Some(e) => e.to_string(),
        None => "{}".to_string(),
    }
}

#[get("/<event_type>/<round>/<num>", format = "application/json")]
fn event_type_round_num(event_type: String, round: u64, num: u64) -> String {
    info!("Query = '({}, {}, {})'", event_type, round, num);
    let data = json_data::DATA.read().unwrap();
    match data.events_type_round_num.get(&(event_type, round, num)) {
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
