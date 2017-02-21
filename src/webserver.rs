use rocket;
use rocket::response::content;
use data::json_data;

#[get("/", format = "application/json")]
fn events() -> content::JSON<String> {
    let data = json_data::DATA.read().unwrap();
    let resp = data.events.clone();
    content::JSON(resp)
}

#[get("/<event_type>", format = "application/json")]
fn event_type(event_type: &str) -> content::JSON<String> {
    info!("Query = '{}'", event_type);
    let data = json_data::DATA.read().unwrap();
    //TODO replace with proper percent decoding.
    let key = event_type.replace("%20", " ");
    let resp = match data.events_type.get(&key) {
        Some(e) => e.to_string(),
        None => "{}".to_string(),
    };
    content::JSON(resp)
}

#[get("/<event_type>/<round>", format = "application/json")]
fn event_type_round(event_type: String, round: u64) -> content::JSON<String> {
    info!("Query = '({}, {})'", event_type, round);
    let data = json_data::DATA.read().unwrap();
    let resp = match data.events_type_round.get(&(event_type, round)) {
        Some(e) => e.to_string(),
        None => "{}".to_string(),
    };
    content::JSON(resp)
}

#[get("/<event_type>/<round>/<num>", format = "application/json")]
fn event_type_round_num(event_type: String, round: u64, num: u64) -> content::JSON<String> {
    info!("Query = '({}, {}, {})'", event_type, round, num);
    let data = json_data::DATA.read().unwrap();
    let resp = match data.events_type_round_num.get(&(event_type, round, num)) {
        Some(e) => e.to_string(),
        None => "{}".to_string(),
    };
    content::JSON(resp)
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
