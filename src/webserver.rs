use rocket;
use rocket::response::content;
use data::json_data;
use diesel::prelude::*;
use super::database;
use super::model::Event as MEvent;
use super::model::Session as MSession;
use super::model;
use itertools::Itertools;
use serde_json;

#[get("/database")]
fn events() -> content::Json<String> {
    use schema::events::dsl::*;
    use schema::sessions::dsl::*;
    let connection = database::establish_connection();
    let my_events: Vec<(MEvent, Option<MSession>)> = events.left_join(sessions).load(&connection).expect("Error loading events");

    let mut cevents = Vec::new();
    for (key,group) in my_events.iter().group_by(|t| t.0.id).into_iter() {
        for &(ref ev, ref session) in group {
            let mut msessions = Vec::new();
            if session.is_some() {
                let s = session.as_ref().unwrap().clone();
                msessions.push(s);
            }
            let e = model::from_model(ev.clone(), msessions);
            cevents.push(e);
        }
        println!("EVENT = {:?}", cevents);
    }
    let json = serde_json::to_string(&cevents).unwrap();
    content::Json(json)
}

#[get("/database/<id>")]
fn events(id: i32) -> content::Json<String> {
    use schema::events::dsl::*;
    use schema::sessions::dsl::*;
    let connection = database::establish_connection();
    let model_event = events.find(id).first::<MEvent>(&conn).expect("Can't load event");
    let model_sessions = sesssions.belonging_to(&e).load(&conn).expected("Can't load sessions");

    let e = model::from_model(model_event, model_sessions);
    let json = serde_json::to_string(&e).unwrap();
    content::Json(json)
}

pub fn start() {
    rocket::ignite()
        .mount("/events", routes![
               events,
               events_id,
               // events,
               // event_type,
               // event_type_round,
               // event_type_round_num,
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
        let mut req = MockRequest::new(Method::Get, "/events").header(ContentType::Json);
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
            let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::Json);
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
            let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::Json);
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
            let mut req = MockRequest::new(Method::Get, &get.replace(" ","%20")).header(ContentType::Json);
            let response = req.dispatch_with(&rocket);
            let expected = match json_data::DATA.read().unwrap().events_type_round_num.get(&key) {
                None => "{}".to_string(),
                Some(x) => x.to_string()
            };
            assert_status_and_string(response, Status::Ok, expected);
        }
    }
}
