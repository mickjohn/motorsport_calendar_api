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
               event_type_round_num,
               ])
        .launch();
}

#[cfg(test)]
mod test {
    use rocket::testing::MockRequest;
    use rocket::http::{Status, Method, ContentType};
    use rocket::{Response, Rocket};
    use serde_json;
    use motorsport_calendar_common::event::Event;
    use super::*;
    use super::super::event_loader;
    use super::super::data::json_data;

    const TEST_DATA_DIR: &'static str = "test/data";

    fn init_test_data() -> Vec<Event> {
        let events = event_loader::load_events_from_yml_file(&format!("{}/test_data.yml", TEST_DATA_DIR)).unwrap();
        json_data::init(&events);
        events
    }

    fn init_rocket() -> Rocket {
        rocket::ignite()
            .mount("/events", routes![
                   events,
                   event_type,
                   event_type_round,
                   event_type_round_num,
                   ])
    }

    fn assert_status_and_string(mut response: Response,status: Status, s: String) {
        assert_eq!(response.status(), status);
        let body_string = response.body().and_then(|b| b.into_string()).unwrap();
        assert_eq!(body_string, s);
    }

    #[test]
    fn test_get_events() {
        let test_events = init_test_data();
        let rocket = init_rocket();
        let mut req = MockRequest::new(Method::Get, "/events").header(ContentType::JSON);
        let response = req.dispatch_with(&rocket);
        let expected = serde_json::to_string(&test_events).unwrap();
        assert_status_and_string(response, Status::Ok, expected);
    }

    #[test]
    fn test_get_events_by_type() {
        init_test_data();
        let rocket = init_rocket();
        for sport_type in vec!["DTM", "GP2", "Formula 1", "doesnt_exist"] {
            let get = format!("/events/{}", sport_type);
            let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::JSON);
            let response = req.dispatch_with(&rocket);
            let expected = match json_data::DATA.read().unwrap().events_type.get(sport_type) {
                None => "{}".to_string(),
                Some(x) => x.to_string()
            };
            assert_status_and_string(response, Status::Ok, expected);
        }
    }
    
    
    #[test]
    fn test_get_events_by_type_and_round() {
        init_test_data();
        let rocket = init_rocket();
        let keys = vec![
            ("DTM".to_string(), 0),
            ("DTM".to_string(), 1),
            ("DTM".to_string(), 2),
            ("DTM".to_string(), 3),
            ("GP2".to_string(), 999),
            ("GP2".to_string(), 1),
            ("GP2".to_string(), 2),
            ("Formula 1".to_string(), 0),
            ("Formula 1".to_string(), 1),
            ];

        for key in keys {
            let get = format!("/events/{}/{}", key.0, key.1);
            let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::JSON);
            let response = req.dispatch_with(&rocket);
            let expected = match json_data::DATA.read().unwrap().events_type_round.get(&key) {
                None => "{}".to_string(),
                Some(x) => x.to_string()
            };
            assert_status_and_string(response, Status::Ok, expected);
        }
    }

    #[test]
    fn test_get_events_by_type_and_round_and_session() {
        init_test_data();
        let rocket = init_rocket();
        let keys = vec![
            ("DTM".to_string(), 0, 1),
            ("DTM".to_string(), 0, 2),
            ("DTM".to_string(), 1, 0),
            ("DTM".to_string(), 1, 1),
            ("DTM".to_string(), 1, 2),
            ("DTM".to_string(), 1, 3),
            ("DTM".to_string(), 2, 1),
            ("DTM".to_string(), 2, 2),
            ("DTM".to_string(), 2, 30),
            ("Formula 1".to_string(), 0, 1),
            ("Formula 1".to_string(), 1, 2),
            ("Formula 1".to_string(), 1, 3),
            ("Formula 1".to_string(), 1, 444),
            ];

        for key in keys {
            let get = format!("/events/{}/{}/{}", key.0, key.1, key.2);
            let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::JSON);
            let response = req.dispatch_with(&rocket);
            let expected = match json_data::DATA.read().unwrap().events_type_round_num.get(&key) {
                None => "{}".to_string(),
                Some(x) => x.to_string()
            };
            assert_status_and_string(response, Status::Ok, expected);
        }
    }
}
